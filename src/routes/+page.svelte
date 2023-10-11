<script lang="ts">
	// import Menu from '$lib/Menu.svelte';
	import { FileDropzone } from '@skeletonlabs/skeleton';
	import '@fortawesome/fontawesome-free/js/all.js';
	import '@fortawesome/fontawesome-free/css/all.css';
	import Icon from '@iconify/svelte';
	import outlineFileUpload from '@iconify/icons-ic/outline-file-upload';
	import successCircleOutline from '@iconify/icons-mdi/success-circle-outline';
	import ChiSquare from '$lib/ChiSquare.svelte';

	let text_input_message = '<strong>Suba un archivo</strong> o arraste';
	let text_meta = 'solo CSV permitido';

	let files: FileList;
	let csv_string: string;
	async function handleDrop(event: Event): Promise<void> {
		files = (event.target as HTMLInputElement).files as FileList;
		const reader = new FileReader();
		text_meta = files[0].name as string;
		text_input_message = 'Archivo cargado';

		const file: File = files[0];
		reader.onload = (event) => {
			csv_string = event.target?.result as string;
			// console.log('heil: ' + csv_string);
		};
		reader.onerror = (error) => {
			console.error(error);
		};
		reader.readAsText(file);
		// console.log(csv_string);
	}
</script>

<div>
	<h1 class="py-4 text-4xl font-bold text-center">Prueba de Chi Cuadrado</h1>
	<FileDropzone
		name="file_upload"
		class="m-auto my-5 w-96 h-30"
		accept=".csv"
		bind:files
		on:change={handleDrop}
	>
		<svelte:fragment slot="lead">
			<div class="flex justify-center">
				{#if files && files.length > 0}
					<Icon icon={successCircleOutline} class="text-3xl" />
				{:else}
					<Icon icon={outlineFileUpload} class="text-3xl" />
				{/if}
			</div>
		</svelte:fragment>

		<div slot="message">{@html text_input_message}</div>
		<svelte:fragment slot="meta">{text_meta}</svelte:fragment>
	</FileDropzone>
	<!-- <Menu {csv_string} /> -->
	<ChiSquare {csv_string} />
</div>
