import type { GraphDistributionValues } from './interfacesGraphDistribution';

// export interface Estadistico {
//   [index: number]: number;
// }
// export interface FrecuenciaEsperada {
//   [index: number]: number;
// }
// export interface FrecuenciaObservada {
//   [index: number]: number;
// }

export interface ResponseData {
  estadistico: { [index: number]: number };
  frecuencia_esperada: { [index: number]: number };
  frecuencia_observada: { [index: number]: number };
  intervalo: { [index: number]: [number, number] };
  poisson: { [index: number]: number };
  total_estadistico: number;
  total_estadistico_tabla: number;
  chi_square_values: GraphDistributionValues;
}
export interface SourceDataItem {
  indice: number;
  intervalo: string;
  poison: number;
  estadistico: number;
  frecuencia_esperada: number;
  frecuencia_observada: number;
  error: number;
}
