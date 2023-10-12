<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import igorPro from '@iconify/icons-file-icons/igor-pro';
	import Icon from '@iconify/svelte';
	import numberOutlined from '@iconify/icons-ant-design/number-outlined';

	import checkCorrect from '@iconify/icons-icon-park-outline/check-correct';

	import characterDecimal from '@iconify/icons-carbon/character-decimal';
	import { Table, tableMapperValues, type TableSource } from '@skeletonlabs/skeleton';
	import type { ResponseData, SourceDataItem } from './interfacesChi';
	// import HuecosChart from './ChartJs/HuecosChart.svelte';
	import ChiSquareGraph from './ChiSquareGraph.svelte';
	import HistogramGraph from './HistogramGraph.svelte';

	export let csv_string: string;

	let response_data: ResponseData;
	let nivel = '';
	let validInputs: boolean = false;
	let showResults: boolean = false;
	let sourceData: SourceDataItem[] = [];
	let estadistico_tabla_graph: [number, number];
	let intervalo_data: string[];
	let frecuencia_observada_data: number[];

	async function prueba_chi_square(nivel_confianza: number) {
		return (await invoke('chi_square', {
			data_string: csv_string,
			nivel_confianza: nivel_confianza
		})) as Object;
	}

	async function btn_prueba_click() {
		if (validInputs) {
			response_data = (await prueba_chi_square(parseFloat(nivel))) as ResponseData;
			showResults = true;

			// const estadistico = response_data.estadistico;
			// const frecuencia_esperada = response_data.frecuencia_esperada;
			// const frecuencia_observada = response_data.frecuencia_observada;
			sourceData = [];
			intervalo_data = [];
			frecuencia_observada_data = [];
			for (let i = 0; i < Object.keys(response_data.estadistico).length; i++) {
				let intervalo_string: string =
					i === 0
						? '[ ' + response_data.intervalo[i][0] + ' - ' + response_data.intervalo[i][1] + ' ]'
						: '< ' + response_data.intervalo[i][0] + ' - ' + response_data.intervalo[i][1] + ' ]';
				sourceData.push({
					indice: i,
					estadistico: response_data.estadistico[i],
					frecuencia_esperada: response_data.frecuencia_esperada[i],
					frecuencia_observada: response_data.frecuencia_observada[i],
					intervalo: intervalo_string,
					poisson: response_data.poisson[i]
					// error: response_data.error[i]
				});
				intervalo_data.push(intervalo_string);
				frecuencia_observada_data.push(response_data.frecuencia_observada[i]);
			}
			estadistico_tabla_graph = [parseFloat(nivel), response_data.total_estadistico_tabla];

			// console.log(intervalo_data);
			// console.log(frecuencia_observada_data);

			console.log(response_data);

			// console.log(sourceData);

			// const tableDataNew: TableSource = (tableData = tableDataNew);
			// console.log(tableData);
		}
	}

	let getTableData: TableSource;

	$: {
		// getTableData = null;
		// let tableDataNew: TableSource = { head: [], body: [], meta: [], foot: [] };
		if (response_data && response_data.total_estadistico !== undefined) {
			getTableData = {
				head: [
					'Intervalo',
					'Frecuencia Observada',
					'Poidson p(x)',
					'Frecuencia Esperada',
					'Estadistico'
				],
				body: tableMapperValues(sourceData, [
					'intervalo',
					'frecuencia_observada',
					'poisson',
					'frecuencia_esperada',
					'estadistico'
				]),
				meta: tableMapperValues(sourceData, ['position']),
				foot: ['Total Estadistico', '', '', '', response_data.total_estadistico.toString()]
			};
			// console.log(getTableData);
		}
		// getTableData = tableDataNew;
	}

	let validar_datos = () => {
		const parseNivel = parseFloat(nivel);
		if (!isNaN(parseNivel) && parseNivel >= 0 && parseNivel <= 1 && csv_string != undefined) {
			validInputs = true;
			// response_data = await prueba_huecos(parseNivel, parseA, parseB);
		} else {
			validInputs = false;
		}
	};
	// $: {
	// validInputs = validar_datos();
	// console.log('validInputs: ', validInputs);
	// validInputs = true;
	// }
</script>

<div class="flex relative flex-col gap-2 items-center max-w-xl">
	<label class="flex flex-col order-1 w-full label">
		<span class="">Nivel de confianza</span>
		<input
			bind:value={nivel}
			class="flex-grow w-full h-9 appearance-none input"
			type="text"
			on:change={validar_datos}
			placeholder="Nivel de confianza"
		/>
		<div class="flex flex-row items-center text-error-400">
			<!-- svelte-ignore empty-block -->
			{#if nivel === ''}{:else if isNaN(parseFloat(nivel))}
				<Icon icon={numberOutlined} />
				<p>Ingrese un valor numerico</p>
			{:else if parseFloat(nivel) < 0 || parseFloat(nivel) > 1}
				<Icon icon={characterDecimal} />
				<p>El valor debe estar entre 0 y 1</p>
			{:else}
				<div class="items-center text-secondary-300-600-token">
					<Icon icon={checkCorrect} />
					<p>Correcto</p>
				</div>
			{/if}
		</div>
	</label>
	<button
		disabled={!validInputs}
		on:click={btn_prueba_click}
		class="order-3 items-center m-3 btn variant-filled-surface"
	>
		<Icon class="text-2xl" icon={igorPro} />
		<span class="text-1xl">Realizar Prueba</span>
	</button>
</div>
{#if showResults}
	<div class="order-4 w-fit">
		<p class="text-2xl font-bold">Resultado</p>
		<Table interactive={true} source={getTableData} />
		<!-- <div class="p-3 max-w-full rounded border-gray-300 bg-primary-100"> -->
		<!-- <HuecosChart {sourceData} /> -->
		<!-- </div> -->
		<br />
		<div class="flex flex-col p-3 space-y-3 max-w-full rounded border-gray-300 bg-primary-100">
			{#key response_data}
				<div class="m-3">
					<ChiSquareGraph
						chi_square_values={response_data.chi_square_values}
						estadistico_tabla={estadistico_tabla_graph}
					/>
				</div>
				<br />
				<div class="m-3">
					<HistogramGraph
						intervalo={intervalo_data}
						frecuencia_observada={frecuencia_observada_data}
					/>
				</div>
			{/key}
		</div>
		<div class="flex items-center p-3">
			{#if response_data.total_estadistico_tabla >= response_data.total_estadistico}
				<p class="font-bold text-success-300">
					El estadistico de prueba es {response_data.total_estadistico} &le; {response_data.total_estadistico_tabla}
					<br />
					No podemos rechazar la hipotesis
				</p>
			{:else}
				<p class="font-bold text-error-300">
					El estadistico de prueba es {response_data.total_estadistico} &ge; {response_data.total_estadistico_tabla}
					<br />
					Podemos rechazar la hipotesis
				</p>{/if}
		</div>
	</div>
{/if}
