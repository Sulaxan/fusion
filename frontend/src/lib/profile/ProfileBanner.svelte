<script>
	import { summonerName } from "$lib/stores";
</script>

<!-- todo: replace banner with actual banner -->
<div
	class="w-full h-64 2xl:h-72 flex flex-col-reverse"
	style="
        background-image: url('http://ddragon.leagueoflegends.com/cdn/img/champion/splash/Akali_2.jpg');
        background-repeat: no-repeat;
        background-size: cover;
        background-position: center top"
>
	<div class="m-5 p-2 bg-base-300 opacity-90 text-base-content w-72">
		<div class="flex items-end">
			<h1 class="text-5xl font-bold">{$summonerName}</h1>
			<div class="flex flex-col text-right mx-2">
				<p class="text-xs">NA</p>
				<p class="text-sm">#0989</p>
			</div>
		</div>
		<div class="flex items-end w-64 justify-between">
			<p class="text-md mx-1 mt-2 w-1/3">Challenger</p>
			<p class="text-md mx-1 mt-2 w-1/3">| 1057 LP</p>
			<p class="text-xs font-bold w-1/3">// Solo/Duo</p>
		</div>
		<div class="flex items-end w-64 justify-between">
			<p class="text-md mx-1 mt-1 w-1/3">Challenger</p>
			<p class="text-md mx-1 mt-1 w-1/3">| 900 LP</p>
			<p class="text-xs font-bold w-1/3">// Flex</p>
		</div>
		<div class="flex items-end w-64 justify-between">
			<p class="text-md mx-1 mt-1 w-1/3">Gladiator</p>
			<p class="text-md mx-1 mt-1 w-1/3">| 5000 GR</p>
			<p class="text-xs font-bold w-1/3">// Arena</p>
		</div>
	</div>
</div>
