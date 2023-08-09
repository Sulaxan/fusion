<script lang="ts">
    import "../app.css";
    import Navbar from "$lib/Navbar.svelte";
	import TitleBar from "$lib/TitleBar.svelte";
</script>

<TitleBar />
<Navbar />
<slot />
