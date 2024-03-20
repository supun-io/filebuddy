<script lang="ts">
	import Selector from './Selector.svelte';
	import { selectedFile } from './stores';
</script>

{#if $selectedFile}
	Selected file: {$selectedFile}
{:else}
	<Selector />
{/if}
