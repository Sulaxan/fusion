<script>
	import { isConnected, summonerName } from "$lib/stores";
</script>

<div class="flex gap-x-2">
	<svg viewBox="0 0 10 10" class="w-3 h-3 shadow-xl">
		<circle cx="5" cy="5" r="5" fill={$isConnected ? "#22c55e" : "#ef4444"} />
	</svg>
	{#if $isConnected}
		<p class="hover:underline hover:underline-offset-2">{$summonerName}</p>
		<!-- todo: replace with actual profile picture -->
		<img
			src="https://raw.communitydragon.org/13.14/game/assets/ux/summonericons/profileicon549.png"
			class="mask mask-circle w-12 h-auto"
			alt="Profile"
		/>
	{:else}
		<div
			class="tooltip tooltip-left"
			data-tip="Fusion couldn't find an open League Client. Is your client open? If so, click
            the refresh icon."
		>
			<p>Not connected</p>
		</div>
	{/if}
</div>
