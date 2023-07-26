<script>
	import MiniProfile from "./profile/MiniProfile.svelte";

</script>
<div class="navbar bg-primary shadow-xl flex justify-between text-primary-content">
	<div class="flex items-end">
		<a class="btn btn-ghost normal-case text-xl" href="/">Fusion</a>
		<p class="text-xs py-2">League Companion</p>
	</div>
	<input
		type="text"
		placeholder="Enter champion or player name..."
		class="input bg-base-300 border-primary w-7/12 max-w-2xl rounded-2xl"
	/>
    <MiniProfile />
</div>
