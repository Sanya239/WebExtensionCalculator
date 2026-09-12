import {MENU_ITEM_ID, saveText} from '../storage/extensionStorage.ts'

// same as in manifest.json
const CALCULATE_SELECTION_COMMAND = 'Calculate with extension'

chrome.runtime.onInstalled.addListener(() => {
    chrome.contextMenus.create({
        id: MENU_ITEM_ID,
        title: 'Calculate with extension',
        contexts: ['selection'],
    })
})

async function openCalculatorWithText(text?: string) {
    const selectedText = text?.trim()
    if (!selectedText) return

    await saveText(selectedText)
    await chrome.action.openPopup()
}

async function readSelectionFromActiveTab() {
    const [activeTab] = await chrome.tabs.query({
        active: true,
        lastFocusedWindow: true,
    })

    if (activeTab?.id === undefined) return ''

    try {
        const [{result = ''}] = await chrome.scripting.executeScript({
            target: {tabId: activeTab.id},
            func: () => window.getSelection()?.toString().trim() ?? '',
        })

        return result
    } catch {
        // Chrome does not allow script injection into internal browser pages.
        return ''
    }
}

chrome.contextMenus.onClicked.addListener(async (info) => {
    if (info.menuItemId !== MENU_ITEM_ID) return

    await openCalculatorWithText(info.selectionText)
})

chrome.commands.onCommand.addListener(async (command) => {
    if (command !== CALCULATE_SELECTION_COMMAND) return

    const selectedText = await readSelectionFromActiveTab()
    await openCalculatorWithText(selectedText)
})
