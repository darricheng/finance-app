<script lang="ts">
  // Skeleton imports
  // The ordering of these imports is critical to your app working properly
  import '@skeletonlabs/skeleton/themes/theme-modern.css';
  // If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
  import '@skeletonlabs/skeleton/styles/skeleton.css';
  // Most of your app wide CSS should be put in this file
  import '../app.postcss';

  // Other imports
  import { AppShell, AppBar, Modal, TabGroup, TabAnchor } from '@skeletonlabs/skeleton';
  import { invoke } from '@tauri-apps/api/tauri';
  import FormModal from '$lib/components/FormModal.svelte';

  import type { ModalComponent } from '@skeletonlabs/skeleton';

  const modalComponentRegistry: Record<string, ModalComponent> = {
    formModal: {
      ref: FormModal,
      // Provide a template literal for the default component slot
      slot: '<p>Skeleton</p>',
    },
  };

  async function printState() {
    invoke('print_state');
  }
</script>

<Modal components={modalComponentRegistry} />
<AppShell>
  <svelte:fragment slot="header">
    <AppBar>
      <svelte:fragment slot="lead">
        <TabGroup
          justify="justify-center"
          active="variant-filled-primary"
          hover="hover:variant-soft-primary"
          flex="flex-1 lg:flex-none"
          rounded=""
          border=""
          class="bg-surface-100-800-token w-full"
        >
          <TabAnchor href="/">
            <svelte:fragment>Home</svelte:fragment>
          </TabAnchor>
          <TabAnchor href="/monthly-view">
            <svelte:fragment>Monthly</svelte:fragment>
          </TabAnchor>
          <TabAnchor href="/budget">
            <svelte:fragment>Budget</svelte:fragment>
          </TabAnchor>
          <TabAnchor href="/income">
            <svelte:fragment>Income</svelte:fragment>
          </TabAnchor>
        </TabGroup>
      </svelte:fragment>
      <svelte:fragment slot="trail">
        <TabGroup
          justify="justify-center"
          active="variant-filled-primary"
          hover="hover:variant-soft-primary"
          flex="flex-1 lg:flex-none"
          rounded=""
          border=""
          class="bg-surface-100-800-token w-full"
        >
          <TabAnchor on:click={printState}>
            <svelte:fragment>PRINT STATE TO TERMINAL</svelte:fragment>
          </TabAnchor>
          <TabAnchor href="/data-input">
            <svelte:fragment>Data Input</svelte:fragment>
          </TabAnchor>
        </TabGroup>
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <slot />
</AppShell>
