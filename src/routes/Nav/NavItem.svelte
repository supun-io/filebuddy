<script lang="ts">
	export let name: string;
</script>

<div class="nav-item">
	{name}
</div>

<style>
	.nav-item {
		display: block;
		padding: 12px 15px;
		background-color: var(--accent);
		border-radius: 5px;
		font-weight: 600;
		cursor: pointer;
	}
</style>
