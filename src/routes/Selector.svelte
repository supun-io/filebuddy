<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { selectedFile } from './stores';

	async function handleClick() {
		const selected = await open({
			multiple: false
		});
		if (typeof selected === 'string') selectedFile.set(selected);
	}
</script>

<div class="wrap">
	<button class="input" on:click={handleClick}>Click to select a file</button>
</div>

<style>
	.wrap {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100vh;
	}

	input {
		display: none;
	}

	.input {
		width: 400px;
		height: 150px;
		border: 2px dashed var(--accent);
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		border-radius: 5px;
	}
	.input:hover {
		background-color: var(--secondary);
	}
</style>
