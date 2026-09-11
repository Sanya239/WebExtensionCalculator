export type CalculationErrorType = 'parse_error' | 'evaluation_error'

export type CalculateResponse =
  | { result: number }
  | {
      error_type: CalculationErrorType
      message: string
      position: number
    }

export type HistoryEntry =
  | {
      expression: string
      result: number
      timestamp: string
    }
  | {
      expression: string
      error_type: CalculationErrorType
      message: string
      position: number
      timestamp: string
    }

export type ApiErrorResponse = {
  error_type: 'invalid_request' | 'internal_error'
  message: string
}

export type CalculatorApi = {
  postCalcExpression(expression: string): Promise<CalculateResponse>
  getHistory(limit?: number): Promise<HistoryEntry[]>
}
