<script lang="ts">
	import { Bar } from 'svelte-chartjs';
	import { Chart, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js';
	export let intervalo: string[];
	export let frecuencia_observada: number[];
	Chart.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

	let data = {
		laberls: intervalo,
		datasets: [
			{
				label: 'Histograma de la frecuencia observada',
				data: frecuencia_observada,
				backgroundColor: [
					'rgba(255, 134,159,0.4)',
					'rgba(98,  182, 239,0.4)',
					'rgba(255, 218, 128,0.4)',
					'rgba(113, 205, 205,0.4)',
					'rgba(170, 128, 252,0.4)',
					'rgba(255, 177, 101,0.4)'
				],
				borderColor: [
					'rgba(255, 134, 159, 1)',
					'rgba(98,  182, 239, 1)',
					'rgba(255, 218, 128, 1)',
					'rgba(113, 205, 205, 1)',
					'rgba(170, 128, 252, 1)',
					'rgba(255, 177, 101, 1)'
				],
				borderWidth: 2
			}
		]
	};
	let options = {
		responsive: true
	};
</script>

<Bar {data} {options} />
