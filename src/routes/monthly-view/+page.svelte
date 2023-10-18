<script lang="ts">
  import { onMount } from 'svelte';
  import Chart from 'chart.js/auto';
  import ChartDataLabels from 'chartjs-plugin-datalabels';

  // Element bindings
  let chartCanvas: HTMLCanvasElement;

  // NOTE: Mock data for the chart
  let budgetBreakdown = [
    { category: 'Food', amount: 180 },
    { category: 'Transport', amount: 150 },
    { category: 'Category X', amount: 300 },
    { category: 'Category Y', amount: 300 },
  ];
  let financeData = [
    { category: 'Food', amount: 111.9 },
    { category: 'Transport', amount: 111.9 },
    { category: 'Category X', amount: 333.9 },
    { category: 'Category Y', amount: 111.9 },
    { category: 'Category Z', amount: 333.9 },
  ];

  // Chart data
  let chartValues = financeData.map((data) => data.amount);
  let chartLabels = financeData.map((data) => data.category);
  let chartColors = financeData.map((data) => {
    const amountSpent = data.amount;
    const budgetedAmount = budgetBreakdown.find(
      (element) => element.category === data.category
    )?.amount;
    if (!budgetedAmount) return 'black'; // there is no such budget category
    if (amountSpent <= budgetedAmount) return 'green'; // within budget!
    else return 'red'; // spent too much
  });

  // Render the chart
  onMount(async () => {
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
