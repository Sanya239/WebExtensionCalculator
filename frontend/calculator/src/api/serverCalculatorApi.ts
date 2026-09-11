import type {
  ApiErrorResponse,
  CalculateResponse,
  CalculatorApi,
  HistoryEntry,
} from './types.ts'

const API_URL = (import.meta.env.VITE_API_URL ?? 'http://localhost:8080').replace(/\/$/, '')

async function request<T>(path: string, options: RequestInit): Promise<T> {
  const response = await fetch(`${API_URL}${path}`, {
    ...options,
    headers: {
      Accept: 'application/json',
      ...options.headers,
    },
  })

  const data = (await response.json().catch(() => null)) as T | ApiErrorResponse | null

  if (!response.ok) {
    const message = data && typeof data === 'object' && 'message' in data
      ? data.message
      : `Request failed with status ${response.status}`

    throw new Error(message)
  }

  if (data === null) {
    throw new Error('Server returned an empty response')
  }

  return data as T
}

export const serverCalculatorApi: CalculatorApi = {
  postCalcExpression(expression: string): Promise<CalculateResponse> {
    return request('/calc_expression', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ expression }),
    })
  },

  getHistory(limit = 100): Promise<HistoryEntry[]> {
    const params = new URLSearchParams({ limit: String(limit) })
    return request(`/get_history?${params}`, { method: 'GET' })
  },
}
