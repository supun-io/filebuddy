<script>
	import Nav from './Nav/Nav.svelte';
	import '../app.css';
	import '@fontsource/merriweather';
</script>

<slot />

<!-- <main>
	<Nav />
	<div id="content">
		<slot />
	</div>
</main> -->

<style>
	main {
		display: flex;
		width: 100vw;
		height: 100vh;
		background-color: #fffff9;
	}
</style>
