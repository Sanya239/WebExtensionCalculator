import { MENU_ITEM_ID, saveText } from '../storage/extensionStorage.ts'

chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: MENU_ITEM_ID,
    title: 'Calculate with extension',
    contexts: ['selection'],
  })
})

chrome.contextMenus.onClicked.addListener(async (info) => {
  if (info.menuItemId !== MENU_ITEM_ID) return

  const text = info.selectionText?.trim()
  if (!text) return

  await saveText(text)
})
