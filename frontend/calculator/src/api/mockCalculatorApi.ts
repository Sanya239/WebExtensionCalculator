import type {
  CalculationErrorType,
  CalculatorApi,
  HistoryEntry,
} from './types.ts'

const STORAGE_KEY = 'calculator.mock.history'
const MOCK_DELAY_MS = 180

type Token = {
  type: 'number' | 'operator' | 'left-parenthesis' | 'right-parenthesis' | 'end'
  value: string
  position: number
}

class MockCalculationError extends Error {
  readonly errorType: CalculationErrorType
  readonly position: number

  constructor(errorType: CalculationErrorType, message: string, position: number) {
    super(message)
    this.errorType = errorType
    this.position = position
  }
}

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

function tokenize(expression: string) {
  const tokens: Token[] = []
  let position = 0

  while (position < expression.length) {
    const character = expression[position]

    if (/\s/.test(character)) {
      position += 1
      continue
    }

    if (/\d|\./.test(character)) {
      const start = position
      let decimalPoints = 0

      while (position < expression.length && /\d|\./.test(expression[position])) {
        if (expression[position] === '.') decimalPoints += 1
        position += 1
      }

      const value = expression.slice(start, position)
      if (decimalPoints > 1 || !/\d/.test(value)) {
        throw new MockCalculationError('parse_error', `Invalid number at position ${start}`, start)
      }

      tokens.push({ type: 'number', value, position: start })
      continue
    }

    if ('+-*/'.includes(character)) {
      tokens.push({ type: 'operator', value: character, position })
      position += 1
      continue
    }

    if (character === '(') {
      tokens.push({ type: 'left-parenthesis', value: character, position })
      position += 1
      continue
    }

    if (character === ')') {
      tokens.push({ type: 'right-parenthesis', value: character, position })
      position += 1
      continue
    }

    throw new MockCalculationError(
      'parse_error',
      `Unexpected character "${character}" at position ${position}`,
      position,
    )
  }

  tokens.push({ type: 'end', value: '', position: expression.length })
  return tokens
}

function evaluate(expression: string) {
  const tokens = tokenize(expression)
  let currentIndex = 0

  function currentToken() {
    return tokens[currentIndex]
  }

  function advance() {
    const token = currentToken()
    currentIndex += 1
    return token
  }

  function ensureFinite(value: number, position: number) {
    if (!Number.isFinite(value)) {
      throw new MockCalculationError('evaluation_error', 'Result is not a finite number', position)
    }

    return Object.is(value, -0) ? 0 : value
  }

  function parsePrimary(): number {
    const token = currentToken()

    if (token.type === 'number') {
      advance()
      return ensureFinite(Number(token.value), token.position)
    }

    if (token.type === 'left-parenthesis') {
      advance()
      const value = parseAddition()

      if (currentToken().type !== 'right-parenthesis') {
        throw new MockCalculationError(
          'parse_error',
          `Expected ")" at position ${currentToken().position}`,
          currentToken().position,
        )
      }

      advance()
      return value
    }

    throw new MockCalculationError(
      'parse_error',
      `Expected a number at position ${token.position}`,
      token.position,
    )
  }

  function parseUnary(): number {
    const token = currentToken()

    if (token.type === 'operator' && (token.value === '+' || token.value === '-')) {
      advance()
      const value = parseUnary()
      return token.value === '-' ? -value : value
    }

    return parsePrimary()
  }

  function parseMultiplication(): number {
    let value = parseUnary()

    while (
      currentToken().type === 'operator'
      && (currentToken().value === '*' || currentToken().value === '/')
    ) {
      const operator = advance()
      const right = parseUnary()

      if (operator.value === '/' && right === 0) {
        throw new MockCalculationError('evaluation_error', 'Division by zero', operator.position)
      }

      value = operator.value === '*' ? value * right : value / right
      value = ensureFinite(value, operator.position)
    }

    return value
  }

  function parseAddition(): number {
    let value = parseMultiplication()

    while (
      currentToken().type === 'operator'
      && (currentToken().value === '+' || currentToken().value === '-')
    ) {
      const operator = advance()
      const right = parseMultiplication()
      value = operator.value === '+' ? value + right : value - right
      value = ensureFinite(value, operator.position)
    }

    return value
  }

  if (!expression.trim()) {
    throw new MockCalculationError('parse_error', 'Expression is empty', 0)
  }

  const result = parseAddition()
  const remainingToken = currentToken()

  if (remainingToken.type !== 'end') {
    throw new MockCalculationError(
      'parse_error',
      `Unexpected token "${remainingToken.value}" at position ${remainingToken.position}`,
      remainingToken.position,
    )
  }

  return result
}

export const mockCalculatorApi: CalculatorApi = {
  async postCalcExpression(expression) {
    await wait()
    const timestamp = new Date().toISOString()
    const history = readHistory()

    try {
      const result = evaluate(expression)
      writeHistory([...history, { expression, result, timestamp }])
      return { result }
    } catch (error) {
      const calculationError = error instanceof MockCalculationError
        ? error
        : new MockCalculationError('evaluation_error', 'Calculation failed', 0)

      const response = {
        error_type: calculationError.errorType,
        message: calculationError.message,
        position: calculationError.position,
      }

      writeHistory([...history, { expression, timestamp, ...response }])
      return response
    }
  },

  async getHistory(limit = 100) {
    await wait()
    const normalizedLimit = Math.max(0, Math.trunc(limit))
    if (normalizedLimit === 0) return []

    return readHistory().slice(-normalizedLimit)
  },
}
