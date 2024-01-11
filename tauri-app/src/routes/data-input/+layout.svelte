<script lang="ts">
  import { AppBar } from '@skeletonlabs/skeleton';
  import SectionHeader from '$lib/components/SectionHeader.svelte';
</script>

<AppBar gridColumns="grid-cols-1" slotDefault="place-self-center" background="none" class="mb-2">
  <SectionHeader>Data Input</SectionHeader>
</AppBar>
<slot />
