<script lang="ts">
  // Props
  /** Exposes parent props to this component. */
  export let parent: any;

  // Stores
  import { getModalStore } from '@skeletonlabs/skeleton';

  const modalStore = getModalStore();

  // Initial form Data
  const formData = {
    name: '',
    amount: '',
    aliases: '',
  };

  // We've created a custom submit function to pass the response and close the modal.
  function onFormSubmit(): void {
    console.log(formData);

    if ($modalStore[0].response) $modalStore[0].response(formData);
    modalStore.close();
  }

  // Base Classes
  const cBase = 'card p-4 w-modal shadow-xl space-y-4';
  const cHeader = 'text-2xl font-bold';
  const cForm = 'border border-surface-500 p-4 space-y-4 rounded-container-token';
</script>

<!-- @component This example creates a simple form modal. -->

{#if $modalStore[0]}
  <div class="modal-example-form {cBase}">
    <header class={cHeader}>{$modalStore[0].title ?? '(title missing)'}</header>
    <!-- submit handles button click, keydown handles pressing enter in any input -->
    <form
      class="modal-form {cForm}"
      on:submit={onFormSubmit}
      on:keydown={(e) => {
        if (e.key === 'Enter') onFormSubmit();
      }}
    >
      <label class="label">
        <span>Category Name</span>
        <input class="input" type="text" bind:value={formData.name} placeholder="e.g. Transport" />
      </label>
      <label class="label">
        <span>Amount</span>
        <input class="input" type="number" bind:value={formData.amount} placeholder="e.g. 400" />
      </label>
      <label class="label">
        <span>Aliases</span>
        <input
          class="input"
          type="text"
          bind:value={formData.aliases}
          placeholder="e.g. public transport, taxi"
        />
      </label>
      <div class="text-right">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}
          >{parent.buttonTextCancel}</button
        >
        <button class="btn {parent.buttonPositive}" type="submit">Submit Form</button>
      </div>
    </form>
  </div>
{/if}
