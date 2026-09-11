import { mockCalculatorApi } from './mockCalculatorApi.ts'
import { serverCalculatorApi } from './serverCalculatorApi.ts'

export type {
  ApiErrorResponse,
  CalculationErrorType,
  CalculateResponse,
  CalculatorApi,
  HistoryEntry,
} from './types.ts'

export const apiMode = import.meta.env.VITE_API_MODE === 'server' ? 'server' : 'mock'

export const calculatorApi = apiMode === 'server'
  ? serverCalculatorApi
  : mockCalculatorApi
