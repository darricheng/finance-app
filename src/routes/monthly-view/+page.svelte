<script lang="ts">
  import { onMount } from 'svelte';
  import Chart from 'chart.js/auto';
  import ChartDataLabels from 'chartjs-plugin-datalabels';
  import { invoke } from '@tauri-apps/api/tauri';

  type ChartData = {
    category: string;
    amount: number;
  }[];

  type Date = {
    month: number;
    year: number;
  };
  type Dates = Date[];

  // Element bindings
  let chartCanvas: HTMLCanvasElement;
  let selectedDateIndex = 0;

  // Dates variable definitions
  let dates: Dates = [];

  let chart: Chart;

  // Chart data variable definitions
  let budgetBreakdown: ChartData = [];
  let expensesData: ChartData = [];

  // Update the chart data
  async function updateData() {
    // Get chart data
    const { budget, expenses }: { budget: ChartData; expenses: ChartData } = await invoke(
      'get_monthly_chart_data',
      { month: dates[selectedDateIndex].month, year: dates[selectedDateIndex].year }
    );

    budgetBreakdown = budget;
    expensesData = expenses;

    chart.data.datasets[0].backgroundColor = expensesData.map((data) => {
      const amountSpent = data.amount;
      const budgetedAmount = budgetBreakdown.find(
        (element) => element.category === data.category
      )?.amount;
      if (!budgetedAmount) return 'black'; // there is no such budget category
      if (amountSpent <= budgetedAmount) return 'green'; // within budget!
      else return 'red'; // spent too much
    });
    chart.data.datasets[0].data = expensesData.map((data) => data.amount);
    chart.data.labels = expensesData.map((data) => data.category);
    chart.update();
  }

  onMount(async () => {
    // Get the dates
    dates = await invoke('get_dates');
    const earliestDate = dates[0];
    dates[selectedDateIndex].month = earliestDate.month;
    dates[selectedDateIndex].year = earliestDate.year;

    // Render the chart
    chart = new Chart(chartCanvas, {
      type: 'bar',
      plugins: [ChartDataLabels], // plugin to show values on the bars
      options: {
        plugins: {
          datalabels: {
            color: 'white',
            font: {
              weight: 'bold',
            },
            formatter: (value) => `$${value}`,
          },
          legend: {
            // Remove the single legend at the top of the chart
            display: false,
          },
        },
      },
      data: {
        labels: [],
        datasets: [
          {
            backgroundColor: [],
            data: [],
          },
        ],
      },
    });

    updateData();
  });
</script>

<div class="grid grid-cols-4 h-full">
  <div class="p-4 flex flex-col gap-4 items-center h-full">
    <h1 class="h1 text-center">Monthly View</h1>
    <select class="select" bind:value={selectedDateIndex} on:change={updateData}>
      {#if dates.length === 0}
        <option value={0}>NO DATA</option>
      {/if}
      {#each dates as date, index}
        <option value={index}>{date.year} - {date.month}</option>
      {/each}
    </select>
    <div class="border-white border-2 rounded-lg p-4 w-full grow">
      {#each budgetBreakdown as category}
        <div class="flex justify-between">
          <p>{category.category}</p>
          <p>${category.amount}</p>
        </div>
      {/each}
    </div>
  </div>
  <div class="p-4 col-span-3">
    <div class="border-white border-2 rounded-lg h-full w-full">
      <canvas bind:this={chartCanvas} id="monthly-view-chart"></canvas>
    </div>
  </div>
</div>
