<script lang="ts">
	import type { GraphDistributionValues } from './interfacesGraphDistribution';
	import { Scatter } from 'svelte-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		CategoryScale
	} from 'chart.js';

	export let chi_square_values: GraphDistributionValues;
	export let estadistico_tabla: [number, number];

	ChartJS.register(Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale);

	const data_estadistico_tabla: [number, number][] = Object.entries(chi_square_values).map(
		([_, value]) => {
			return [value[0], value[1]];
			// console.log(`Key: ${key}, Value: ${value}`);
		}
	);

	// console.log('heil esquizo');
	// console.log(chi_square_values);
	console.log(data_estadistico_tabla);
	console.log(estadistico_tabla);
	// data_estadistico_tabla.forEach((item) => {
	// 	console.log(item[0]);
	// 	console.log(item[1]);
	// });
	let data = {
		labels: data_estadistico_tabla.map((item) => item[0]),
		datasets: [
			{
				label: 'Valores de Chi-Cuadrado',
				data: data_estadistico_tabla.map((item) => item[1]),
				showLine: true,
				backgroundColor: 'rgba(75, 192, 230, 0.2)',
				pointRadius: 0.5
				// cubicInterpolationMode: 'monotone'
			},
			{
				label: 'Estadistico de Tabla',
				data: [{ x: estadistico_tabla[0], y: estadistico_tabla[1] }],
				borderColor: 'rgba(0, 0, 0, 0)',
				borderWidth: 2,
				pointBackgroundColor: 'rgb(255, 0, 0)',
				pointRadius: 5,
				pointHoverRadius: 5,
				borderDash: [5, 5]
			}
		]
	};
	let options = {
		responsive: true
	};
</script>

<Scatter {data} {options} />
