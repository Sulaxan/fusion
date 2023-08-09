<script lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
</script>

<div
	data-tauri-drag-region
	class="titlebar select-none bg-primary sticky top-0 left-0 right-0 flex justify-end h-[40px]"
>
	<div
		class="titlebar-button hover:bg-gray-100 hover:bg-opacity-20"
		id="titlebar-minimize"
		on:click={() => appWindow.minimize()}
	>
		<img class="p-3" src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
	</div>
	<div
		class="titlebar-button hover:bg-gray-100 hover:bg-opacity-20"
		id="titlebar-maximize"
		on:click={() => appWindow.maximize()}
	>
		<img class="p-3" src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
	</div>
	<div
		class="titlebar-button hover:bg-red-500"
		id="titlebar-close"
		on:click={() => appWindow.close()}
	>
		<img class="p-3" src="https://api.iconify.design/mdi:close.svg" alt="close" />
	</div>
</div>
