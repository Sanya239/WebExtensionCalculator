import type {
  ApiErrorResponse,
  CalculateResponse,
  CalculatorApi,
  HistoryEntry,
} from './types.ts'
import { getOrCreateDeviceId } from '../storage/extensionStorage.ts'

const API_URL = (import.meta.env.VITE_API_URL ?? 'http://127.0.0.1:3000').replace(/\/$/, '')

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
  async postCalcExpression(expression: string): Promise<CalculateResponse> {
    const deviceId = await getOrCreateDeviceId()

    return request('/api/calculate', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ expression, device_id: deviceId }),
    })
  },

  async getHistory(limit = 100): Promise<HistoryEntry[]> {
    const deviceId = await getOrCreateDeviceId()
    const params = new URLSearchParams({
      limit: String(limit),
      device_id: deviceId,
    })

    return request(`/api/history?${params}`, { method: 'GET' })
  },
}
