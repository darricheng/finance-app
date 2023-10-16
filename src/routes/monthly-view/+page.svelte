<script lang="ts">
  import Chart from 'chart.js/auto';
  import { onMount } from 'svelte';

  let chartValues = [20, 10, 5, 2, 20, 30, 45];
  let chartLabels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];
  let chartCanvas: HTMLCanvasElement;

  onMount(async () => {
    new Chart(chartCanvas, {
      type: 'bar',
      data: {
        labels: chartLabels,
        datasets: [
          {
            label: 'Revenue',
            backgroundColor: 'rgb(255, 99, 132)',
            borderColor: 'rgb(255, 99, 132)',
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
      <div class="flex justify-between">
        <span>Income</span>
        <span>5000</span>
      </div>
      <div class="flex justify-between">
        <span>Expenses</span>
        <span>5000</span>
      </div>
      <div class="flex justify-between">
        <span>Balance</span>
        <span>5000</span>
      </div>
    </div>
  </div>
  <div class="p-4 col-span-3">
    <div class="border-white border-2 rounded-lg h-full w-full">
      <canvas bind:this={chartCanvas} id="monthly-view-chart"></canvas>
    </div>
  </div>
</div>
