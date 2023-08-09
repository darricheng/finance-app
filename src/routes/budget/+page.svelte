<script lang="ts">
  import { onMount } from 'svelte';
  import { Table, tableMapperValues } from '@skeletonlabs/skeleton';
  import { invoke } from '@tauri-apps/api/tauri';

  import type { TableSource } from '@skeletonlabs/skeleton';
  import type { Budget } from '../../../src-tauri/bindings/Budget';

  let budget: Budget = {
    categories: [],
  };

  const addBudgetCategory = () => {
    invoke('add_new_budget_category', {
      name: 'New Category',
      amount: 1000,
      aliases: ['new', 'category'],
    });
  };

  const getBudget = () => {
    return invoke('get_budget');
  };

  onMount(async () => {
    budget = (await getBudget()) as Budget;
    console.log(budget);
  });

  let table: TableSource;
  $: table = {
    head: ['Category', 'Amount', 'Aliases'],
    body: tableMapperValues(budget.categories, ['name', 'amount', 'aliases']),
  };
</script>

<div class="p-8">
  <div class="grid grid-cols-2 mb-8">
    <h1 class="h1">Budget</h1>
    <div class="flex flex-row-reverse">
      <button type="button" class="btn variant-filled" on:click={addBudgetCategory}
        >+ Add New</button
      >
    </div>
  </div>
  <Table bind:source={table} />
</div>
