import type { CalculatorApi, HistoryEntry } from './types.ts'

const STORAGE_KEY = 'calculator.mock.history'
const MOCK_DELAY_MS = 180

let memoryHistory: HistoryEntry[] = []

function wait() {
  return new Promise<void>((resolve) => setTimeout(resolve, MOCK_DELAY_MS))
}

function readHistory() {
  try {
    const storedHistory = localStorage.getItem(STORAGE_KEY)
    if (!storedHistory) return memoryHistory

    const parsedHistory: unknown = JSON.parse(storedHistory)
    return Array.isArray(parsedHistory) ? parsedHistory as HistoryEntry[] : memoryHistory
  } catch {
    return memoryHistory
  }
}

function writeHistory(history: HistoryEntry[]) {
  memoryHistory = history

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(history))
  } catch {
    // The in-memory fallback keeps mock mode working when storage is unavailable.
  }
}

function evaluate(expression: string) {
  if (!expression.trim()) throw new Error('Expression is empty')

  // Mock mode is only used during local development.
  // oxlint-disable-next-line no-eval
  const result: unknown = globalThis.eval(expression)

  if (typeof result !== 'number' || !Number.isFinite(result)) {
    throw new Error('Expression did not produce a finite number')
  }

  return result
}

export const mockCalculatorApi: CalculatorApi = {
  async postCalcExpression(expression: string) {
    await wait()

    const history = readHistory()
    const timestamp = new Date().toISOString()

    try {
      const result = evaluate(expression)
      writeHistory([...history, { expression, result, timestamp }])
      return { result }
    } catch (error) {
      const response = {
        error_type: 'evaluation_error' as const,
        message: error instanceof Error ? error.message : 'Calculation failed',
        position: 0,
      }

      writeHistory([...history, { expression, timestamp, ...response }])
      return response
    }
  },

  async getHistory(limit = 100) {
    await wait()

    const normalizedLimit = Math.max(0, Math.trunc(limit))
    return normalizedLimit === 0 ? [] : readHistory().slice(-normalizedLimit)
  },
}
