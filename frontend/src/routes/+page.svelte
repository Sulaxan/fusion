<script>
	import NotConnectedDisplay from "$lib/NotConnectedDisplay.svelte";
	import ProfileBanner from "$lib/profile/ProfileBanner.svelte";
	import { isConnected } from "$lib/stores";
</script>

{#if $isConnected}
    <ProfileBanner />
{:else}
	<NotConnectedDisplay />
{/if}
