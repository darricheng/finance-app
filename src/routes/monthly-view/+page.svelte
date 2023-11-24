<script lang="ts">
  import { onMount } from 'svelte';
  import Chart from 'chart.js/auto';
  import ChartDataLabels from 'chartjs-plugin-datalabels';
  import { invoke } from '@tauri-apps/api/tauri';

  type ChartData = {
    category: string;
    amount: number;
  }[];

  // Element bindings
  let chartCanvas: HTMLCanvasElement;

  // Chart data variable definitions
  let budgetBreakdown: ChartData = [];
  let expensesData: ChartData = [
    { category: 'Food', amount: 111.9 },
    { category: 'Transport', amount: 111.9 },
    { category: 'Category X', amount: 333.9 },
    { category: 'Category Y', amount: 111.9 },
    { category: 'Category Z', amount: 333.9 },
  ];
  let chartColors: string[] = [];

  // TODO: Probably have to shift this into updateData() once I get the data from the backend?
  let chartValues = expensesData.map((data) => data.amount);
  let chartLabels = expensesData.map((data) => data.category);

  // Update the chart data
  function updateData(budget: ChartData, expenses: ChartData) {
    budgetBreakdown = budget;
    // NOTE: This is commented out because the backend does not return the expenses data yet
    // expensesData = expenses;

    chartColors = expensesData.map((data) => {
      const amountSpent = data.amount;
      const budgetedAmount = budgetBreakdown.find(
        (element) => element.category === data.category
      )?.amount;
      if (!budgetedAmount) return 'black'; // there is no such budget category
      if (amountSpent <= budgetedAmount) return 'green'; // within budget!
      else return 'red'; // spent too much
    });
  }

  // Render the chart
  onMount(async () => {
    const data: {
      budget: ChartData;
      expenses: ChartData;
    } = await invoke('get_monthly_chart_data', { month: 0 }); // WARN: HARDCODED MONTH
    console.log(data);
    updateData(data.budget, data.expenses);
    new Chart(chartCanvas, {
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
        labels: chartLabels,
        datasets: [
          {
            backgroundColor: chartColors,
            data: chartValues,
          },
        ],
      },
    });
  });
</script>

<div class="grid grid-cols-4 h-full">
  <div class="p-4 flex flex-col gap-4 items-center h-full">
    <h1 class="h1 text-center">Monthly View</h1>
    <select class="select">
      <option value="1">Option 1</option>
      <option value="2">Option 2</option>
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
