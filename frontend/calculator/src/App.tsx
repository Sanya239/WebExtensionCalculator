import { useEffect, useRef, useState } from 'react'
import {
  calculatorApi,
  type CalculateResponse,
  type HistoryEntry,
} from './api/calculatorApi.ts'

const HISTORY_LIMIT = 100

function sortHistory(entries: HistoryEntry[]) {
  return [...entries].sort(
    (left, right) => new Date(left.timestamp).getTime() - new Date(right.timestamp).getTime(),
  )
}

function formatTimestamp(timestamp: string) {
  const date = new Date(timestamp)

  if (Number.isNaN(date.getTime())) {
    return timestamp
  }

  return new Intl.DateTimeFormat('en-GB', {
    day: '2-digit',
    month: '2-digit',
    year: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  }).format(date)
}

function getHistoryResult(entry: HistoryEntry) {
  return 'result' in entry ? String(entry.result) : entry.message
}

function getResponseText(response: CalculateResponse) {
  return 'result' in response ? String(response.result) : response.message
}

function App() {
  const [expression, setExpression] = useState('')
  const [result, setResult] = useState('')
  const [history, setHistory] = useState<HistoryEntry[]>([])
  const [isCalculating, setIsCalculating] = useState(false)
  const historyRef = useRef<HTMLDivElement>(null)
  const inputRef = useRef<HTMLInputElement>(null)

  async function loadHistory() {
    const entries = await calculatorApi.getHistory(HISTORY_LIMIT)
    setHistory(sortHistory(entries))
  }

  useEffect(() => {
    calculatorApi
      .getHistory(HISTORY_LIMIT)
      .then((entries) => setHistory(sortHistory(entries)))
      .catch(() => undefined)
  }, [])

  useEffect(() => {
    const historyElement = historyRef.current

    if (historyElement) {
      historyElement.scrollTop = historyElement.scrollHeight
    }
  }, [history])

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault()

    const submittedExpression = expression.trim()
    if (!submittedExpression || isCalculating) return

    setIsCalculating(true)

    try {
      const response = await calculatorApi.postCalcExpression(submittedExpression)
      setResult(getResponseText(response))
      await loadHistory().catch(() => undefined)
    } catch (error) {
      setResult(error instanceof Error ? error.message : 'Request failed')
    } finally {
      setIsCalculating(false)
      requestAnimationFrame(() => inputRef.current?.select())
    }
  }

  function handleHistoryClick(entry: HistoryEntry) {
    setExpression(entry.expression)
    setResult(getHistoryResult(entry))
  }

  return (
    <main className="flex min-h-dvh items-center justify-center bg-stone-100 p-4 text-stone-950">
      <section className="flex h-[560px] w-full max-w-[420px] flex-col rounded-[2rem] border border-stone-200 bg-white p-5 shadow-[0_16px_45px_rgba(28,25,23,0.08)]">
        <header className="mb-4 flex items-center justify-between px-1">
          <h1 className="text-sm font-semibold tracking-[-0.01em]">Calculator</h1>
          <span className="rounded-full bg-stone-100 px-3 py-1 text-[11px] font-medium text-stone-500">
            {history.length} {history.length === 1 ? 'calculation' : 'calculations'}
          </span>
        </header>

        <div
          ref={historyRef}
          className="min-h-0 flex-1 overflow-y-auto rounded-2xl border border-stone-200 bg-stone-50 px-4"
          aria-label="Calculation history"
        >
          {history.length === 0 ? (
            <div className="flex h-full items-center justify-center text-sm text-stone-400">
              History is empty
            </div>
          ) : (
            <ul>
              {history.map((entry, index) => (
                <li
                  key={`${entry.timestamp}-${entry.expression}-${index}`}
                  className="border-b border-stone-200 py-1 last:border-b-0"
                >
                  <button
                    className="flex w-full items-center gap-3 rounded-xl px-2 py-2.5 text-left outline-none transition hover:bg-white focus-visible:bg-white focus-visible:ring-2 focus-visible:ring-stone-300"
                    type="button"
                    onClick={() => handleHistoryClick(entry)}
                  >
                    <span className="min-w-0 flex-1 truncate text-sm text-stone-600">
                      <span className="font-medium text-stone-950">{entry.expression}</span>
                      <span className="mx-2 text-stone-300">=</span>
                      <span>{getHistoryResult(entry)}</span>
                    </span>
                    <time
                      className="shrink-0 text-[10px] tabular-nums text-stone-400"
                      dateTime={entry.timestamp}
                    >
                      {formatTimestamp(entry.timestamp)}
                    </time>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </div>

        <div className="mt-4 flex flex-col gap-2">
          <form onSubmit={handleSubmit}>
            <label className="sr-only" htmlFor="expression">
              Expression
            </label>
            <input
              ref={inputRef}
              id="expression"
              className="w-full rounded-2xl border border-stone-200 bg-stone-50 px-4 py-4 text-base text-stone-950 outline-none transition placeholder:text-stone-400 focus:border-stone-400 focus:bg-white focus:ring-4 focus:ring-stone-100 disabled:cursor-wait disabled:text-stone-400"
              type="text"
              inputMode="text"
              autoComplete="off"
              autoFocus
              placeholder="Type an expression and press Enter"
              value={expression}
              disabled={isCalculating}
              onChange={(event) => {
                setExpression(event.target.value)
                setResult('')
              }}
            />
            <button className="sr-only" type="submit" tabIndex={-1}>
              Calculate
            </button>
          </form>
          <label className="sr-only" htmlFor="result">
            Result
          </label>
          <input
            id="result"
            className="w-full rounded-2xl border border-stone-200 bg-stone-100 px-4 py-3 text-sm text-stone-600 outline-none placeholder:text-stone-400 focus:ring-4 focus:ring-stone-100"
            type="text"
            placeholder="Result"
            value={result}
            readOnly
            aria-live="polite"
          />
        </div>
      </section>
    </main>
  )
}

export default App
