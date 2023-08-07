<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let expense_data = '';

  async function printState() {
    invoke('print_state');
  }

  async function addExpenses() {
    invoke('add_expenses', { data: expense_data });
  }
</script>

<div class="grid grid-cols-2 h-5/6">
  <div class="p-4 border-r-2 border-white flex flex-col items-center gap-8">
    <h2 class="h2">Expenses</h2>
    <button type="button" class="btn variant-filled" on:click={printState}
      >PRINT STATE TO CONSOLE</button
    >
    <button type="button" class="btn variant-filled" on:click={addExpenses}>+ Add</button>
    <textarea
      class="textarea grow"
      placeholder="Paste data as CSV in the format: date, category, amount, description"
      bind:value={expense_data}
    />
  </div>
  <div class="p-4 flex flex-col items-center gap-8">
    <h2 class="h2">Income</h2>
    <form class="flex flex-col gap-4">
      <label class="label">
        <span>Source</span>
        <input class="input" title="Source" type="text" placeholder="e.g. main job" />
      </label>
      <label class="label">
        <span>Amount</span>
        <input class="input" title="Amount" type="text" placeholder="e.g. 5000" />
      </label>
      <button type="submit" class="btn variant-filled">+ Add</button>
    </form>
  </div>
</div>
