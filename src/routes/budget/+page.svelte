<script lang="ts">
  import { onMount } from 'svelte';
  import { modalStore, Table, tableMapperValues } from '@skeletonlabs/skeleton';
  import { invoke } from '@tauri-apps/api/tauri';

  import type { TableSource, ModalSettings } from '@skeletonlabs/skeleton';
  import type { Budget } from '../../../src-tauri/bindings/Budget';

  interface BudgetCategory {
    name: string;
    amount: number;
    aliases: string;
  }
  // Type guard for BudgetCategory
  // See https://bobbyhadz.com/blog/typescript-type-unknown-is-not-assignable-to-type#using-a-type-guard-when-checking-for-objects
  // Probably overkill here, but something new to try
  function isBudgetCategory(obj: any): obj is BudgetCategory {
    return (
      typeof obj === 'object' &&
      obj !== null &&
      'name' in obj &&
      'amount' in obj &&
      'aliases' in obj
    );
  }

  let budget: Budget = {
    categories: [],
  };

  const addBudgetCategory = () => {
    // Promise is used to get the response from the modal
    new Promise((resolve) => {
      const newBudgetCategoryModal: ModalSettings = {
        type: 'component',
        title: 'New Budget Category',
        component: 'formModal',
        response: (res) => resolve(res),
      };
      modalStore.trigger(newBudgetCategoryModal);
    })
      .then((newCategory) => {
        if (!isBudgetCategory(newCategory)) return;
        newCategory.aliases.trim();
        const re = /\s*(?:,|$)\s*/; // Remove whitespace before and after the comma
        const aliases = newCategory.aliases.split(re);
        invoke('add_new_budget_category', {
          name: newCategory.name,
          amount: newCategory.amount,
          aliases,
        });
      })
      .finally(() => {
        // Refetch the budget and update the table with the new category
        getBudget();
      });
  };

  // Skeleton Labs doesn't seem to provide an appropriate type for the CustomEvent
  // that is passed to the Table's on:selected event handler
  const editRow = (meta: any) => {
    new Promise((resolve) => {
      const editBudgetCategoryModal: ModalSettings = {
        type: 'component',
        title: `Edit Category: ${meta.detail[0]}`,
        component: 'formModal',
        response: (res) => resolve(res),
      };
      modalStore.trigger(editBudgetCategoryModal);
    })
      .then((editCategory) => {
        if (!isBudgetCategory(editCategory)) return;
        editCategory.aliases.trim();
        const re = /\s*(?:,|$)\s*/; // Remove whitespace before and after the comma
        const aliases = editCategory.aliases.split(re);
        invoke('edit_budget_category', {
          name: editCategory.name,
          amount: editCategory.amount,
          aliases,
        });
      })
      .finally(() => {
        // Refetch the budget and update the table with the new category
        getBudget();
      });
  };

  const getBudget = async () => {
    budget = (await invoke('get_budget')) as Budget;
    if (budget.categories.length === 0) {
      budget.categories.push({
        name: 'No Data',
        amount: 0,
        aliases: ['No Data'],
      });
    }
  };

  onMount(async () => {
    getBudget();
  });

  let table: TableSource;
  $: table = {
    head: ['Category', 'Amount', 'Aliases'],
    body: tableMapperValues(budget.categories, ['name', 'amount', 'aliases']),
    meta: tableMapperValues(budget.categories, ['name', 'amount', 'aliases']),
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
  <p>Click on the row to edit it</p>
  <Table bind:source={table} interactive={true} on:selected={editRow} />
</div>
