export const INPUT_STORAGE_KEY = 'Calculator-extension-input'
export const RESULT_STORAGE_KEY = 'Calculator-extension-result'
export const CALCULATE_ON_OPEN_STORAGE_KEY = 'Calculator-extension-calculate-on-open'
export const DEVICE_ID_STORAGE_KEY = 'Calculator-extension-device-id'
export const MENU_ITEM_ID = 'calculate-with-extension'

const DEVICE_ID_PATTERN = /^[A-Za-z0-9_-]{1,128}$/

let deviceIdPromise: Promise<string> | undefined

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

async function writeValues(values: Record<string, string | boolean>) {
  if (hasExtensionStorage()) {
    await chrome.storage.local.set(values)
    return
  }

  if (!hasLocalStorage()) return

  for (const [key, value] of Object.entries(values)) {
    localStorage.setItem(key, String(value))
  }
}

function createDeviceId() {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID()
  }

  return `calculator-${Date.now()}-${Math.random().toString(36).slice(2)}`
}

export function getOrCreateDeviceId(): Promise<string> {
  deviceIdPromise ??= (async () => {
    const values = await readValues([DEVICE_ID_STORAGE_KEY])
    const storedDeviceId = values[DEVICE_ID_STORAGE_KEY]

    if (typeof storedDeviceId === 'string' && DEVICE_ID_PATTERN.test(storedDeviceId)) {
      return storedDeviceId
    }

    const deviceId = createDeviceId()
    await writeValues({ [DEVICE_ID_STORAGE_KEY]: deviceId })
    return deviceId
  })()

  return deviceIdPromise
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
  await writeValues({
    [INPUT_STORAGE_KEY]: text,
    [RESULT_STORAGE_KEY]: '',
    [CALCULATE_ON_OPEN_STORAGE_KEY]: true,
  })
  return { success: true as const }
}

export async function consumeCalculationOnOpen() {
  const values = await readValues([
    INPUT_STORAGE_KEY,
    CALCULATE_ON_OPEN_STORAGE_KEY,
  ])
  const shouldCalculate = values[CALCULATE_ON_OPEN_STORAGE_KEY] === true
    || values[CALCULATE_ON_OPEN_STORAGE_KEY] === 'true'

  if (!shouldCalculate) return null

  await writeValues({ [CALCULATE_ON_OPEN_STORAGE_KEY]: false })

  return typeof values[INPUT_STORAGE_KEY] === 'string'
    ? values[INPUT_STORAGE_KEY]
    : null
}
