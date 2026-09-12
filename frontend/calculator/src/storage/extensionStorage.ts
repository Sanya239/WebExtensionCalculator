export const INPUT_STORAGE_KEY = 'Calculator-extension-input'
export const RESULT_STORAGE_KEY = 'Calculator-extension-result'
export const MENU_ITEM_ID = 'calculate-with-extension'

export type CalculatorFormState = {
  expression: string
  result: string
}

function hasExtensionStorage() {
  return typeof chrome !== 'undefined' && Boolean(chrome.storage?.local)
}

function hasLocalStorage() {
  return typeof localStorage !== 'undefined'
}

async function readValues(keys: string[]): Promise<Record<string, unknown>> {
  if (hasExtensionStorage()) {
    return chrome.storage.local.get(keys)
  }

  if (!hasLocalStorage()) return {}

  return Object.fromEntries(keys.map((key) => [key, localStorage.getItem(key)]))
}

async function writeValues(values: Record<string, string>) {
  if (hasExtensionStorage()) {
    await chrome.storage.local.set(values)
    return
  }

  if (!hasLocalStorage()) return

  for (const [key, value] of Object.entries(values)) {
    localStorage.setItem(key, value)
  }
}

export async function readFormState(): Promise<CalculatorFormState> {
  const values = await readValues([INPUT_STORAGE_KEY, RESULT_STORAGE_KEY])

  return {
    expression: typeof values[INPUT_STORAGE_KEY] === 'string'
      ? values[INPUT_STORAGE_KEY]
      : '',
    result: typeof values[RESULT_STORAGE_KEY] === 'string'
      ? values[RESULT_STORAGE_KEY]
      : '',
  }
}

export async function writeFormState(state: CalculatorFormState) {
  await writeValues({
    [INPUT_STORAGE_KEY]: state.expression,
    [RESULT_STORAGE_KEY]: state.result,
  })
}

export async function saveText(text: string) {
  await writeFormState({ expression: text, result: '' })
  return { success: true as const }
}
