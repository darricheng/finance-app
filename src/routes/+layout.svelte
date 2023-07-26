<script lang="ts">
  // Skeleton imports
  // The ordering of these imports is critical to your app working properly
  import '@skeletonlabs/skeleton/themes/theme-modern.css';
  // If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
  import '@skeletonlabs/skeleton/styles/skeleton.css';
  // Most of your app wide CSS should be put in this file
  import '../app.postcss';

  // Other imports
  import { AppShell, AppBar } from '@skeletonlabs/skeleton';
</script>

<AppShell>
  <svelte:fragment slot="header">
    <AppBar>Hello World</AppBar>
  </svelte:fragment>
  <slot />
</AppShell>
