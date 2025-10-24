# Mapping from Playwright

Legends used in status:

- ❔ = has not yet figure out how it works
- 🗑️ = deprecated by Playwright
- ❌ = to be implemented
- 🚧 = blocked by dependencies
- 🚫 = CDP specific, will not implement
- ✅ = implemented

## APIRequest

| Status | Playwright                | WebDriver Bidi | Note |
| ------ | ------------------------- | -------------- | ---- |
| ❔     | `APIRequest.newContext()` |                |      |

## APIRequestContext

| Status | Playwright                        | WebDriver Bidi | Note |
| ------ | --------------------------------- | -------------- | ---- |
| ❔     | `APIRequesContext.delete()`       |                |      |
| ❔     | `APIRequesContext.dispose()`      |                |      |
| ❔     | `APIRequesContext.fetch()`        |                |      |
| ❔     | `APIRequesContext.get()`          |                |      |
| ❔     | `APIRequesContext.head()`         |                |      |
| ❔     | `APIRequesContext.patch()`        |                |      |
| ❔     | `APIRequesContext.post()`         |                |      |
| ❔     | `APIRequesContext.put()`          |                |      |
| ❔     | `APIRequesContext.storageState()` |                |      |

## APIResponse

| Status | Playweight                 | WebDriver Bidi | Note |
| ------ | -------------------------- | -------------- | ---- |
| ❔     | `APIResponse.body`         |                |      |
| ❔     | `APIResponse.dispose`      |                |      |
| ❔     | `APIResponse.headers`      |                |      |
| ❔     | `APIResponse.headersArray` |                |      |
| ❔     | `APIResponse.json`         |                |      |
| ❔     | `APIResponse.ok`           |                |      |
| ❔     | `APIResponse.status`       |                |      |
| ❔     | `APIResponse.statusText`   |                |      |
| ❔     | `APIResponse.text`         |                |      |
| ❔     | `APIResponse.url`          |                |      |

## Accessibility

> [!NOTE]
>
> Deprecated, lower precedence

| Status | Playweight               | WebDriver Bidi | Note                      |
| ------ | ------------------------ | -------------- | ------------------------- |
| 🗑️     | `Accessibility.snapshot` |                | Deprecated in Playwright. |

## Browser

| Status | Playweight                     | WebDriver Bidi              | Note                                         |
| ------ | ------------------------------ | --------------------------- | -------------------------------------------- |
| ✅     | `Browser.browserType`          | N/A                         | Implemented as property.                     |
| ✅     | `Browser.close`                | `browser.close`             |                                              |
| ✅     | `Browser.contexts`             | `browser.getUserContexts`   | Async instead.                               |
| 🚧     | `Browser.isConnected`          | N/A                         | Check whether internal connection is closed. |
| 🚫     | `Browser.newBrowserCDPSession` | N/A                         | We use WebDriver Bidi, not CDP.              |
| ✅     | `Browser.newContext`           | `browser.createUserContext` |                                              |
| ✅     | `Browser.newPage`              | `browseringContext.create`  |                                              |
| 🚧     | `Browser.removeAllListeners`   | N/A                         | Listeners are internally managed.            |
| 🚫     | `Browser.startTracing`         | N/A                         | Tracing is CDP specific.                     |
| 🚫     | `Browser.stopTracing`          | N/A                         | Tracing is CDP specific.                     |
| 🚧     | `Browser.version`              | `session.new`               | Not yet exposed in upstream `webdriverbidi`. |
| 🚧     | `Browser.on('disconnected')`   | N/A                         | Check whether connection is closed           |

## BrowserContext

| Status | Playweight                                   | WebDriver Bidi | Note                            |
| ------ | -------------------------------------------- | -------------- | ------------------------------- |
| ❌     | `BrowserContext.addCookies`                  |                |                                 |
| ❌     | `BrowserContext.addInitScript`               |                |                                 |
| ❌     | `BrowserContext.browser`                     |                |                                 |
| ❌     | `BrowserContext.clearCookies`                |                |                                 |
| ❌     | `BrowserContext.clearPermissions`            |                |                                 |
| ❌     | `BrowserContext.close`                       |                |                                 |
| ❌     | `BrowserContext.cookies`                     |                |                                 |
| ❌     | `BrowserContext.exposeBinding`               |                |                                 |
| ❌     | `BrowserContext.exposeFunction`              |                |                                 |
| ❌     | `BrowserContext.grantPermissions`            |                |                                 |
| 🚫     | `BrowserContext.newCDPSession`               | N/A            | We use WebDriver Bidi, not CDP. |
| ❌     | `BrowserContext.newPage`                     |                |                                 |
| ❌     | `BrowserContext.pages`                       |                |                                 |
| ❌     | `BrowserContext.removeAllListeners`          |                |                                 |
| ❌     | `BrowserContext.route`                       |                |                                 |
| ❌     | `BrowserContext.routeFromHAR`                |                |                                 |
| ❌     | `BrowserContext.routeWebSocket`              |                |                                 |
| ❌     | `BrowserContext.serviceWorkers`              |                |                                 |
| ❌     | `BrowserContext.setDefaultNavigationTimeout` |                |                                 |
| ❌     | `BrowserContext.setDefaultTimeout`           |                |                                 |
| ❌     | `BrowserContext.setExtraHTTPHeaders`         |                |                                 |
| ❌     | `BrowserContext.setGeolocation`              |                |                                 |
| ❌     | `BrowserContext.setOffline`                  |                |                                 |
| ❌     | `BrowserContext.storageState`                |                |                                 |
| ❌     | `BrowserContext.unroute`                     |                |                                 |
| ❌     | `BrowserContext.unrouteAll`                  |                |                                 |
| ❌     | `BrowserContext.waitForEvent`                |                |                                 |
| ❌     | `BrowserContext.clock`                       |                |                                 |
| ❌     | `BrowserContext.request`                     |                |                                 |
| ❌     | `BrowserContext.tracing`                     |                |                                 |
| ❌     | `BrowserContext.on('close')`                 |                |                                 |
| ❌     | `BrowserContext.on('console')`               |                |                                 |
| ❌     | `BrowserContext.on('dialog')`                |                |                                 |
| ❌     | `BrowserContext.on('page')`                  |                |                                 |
| ❌     | `BrowserContext.on('request')`               |                |                                 |
| ❌     | `BrowserContext.on('requestfailed')`         |                |                                 |
| ❌     | `BrowserContext.on('requestfinished')`       |                |                                 |
| ❌     | `BrowserContext.on('response')`              |                |                                 |
| ❌     | `BrowserContext.on('serviceworker')`         |                |                                 |
| ❌     | `BrowserContext.on('weberror')`              |                |                                 |
| ❌     | `BrowserContext.on('backgroundpage')`        |                |                                 |
| ❌     | `BrowserContext.backgroundPages`             |                |                                 |
| ❌     | `BrowserContext.setHTTPCredentials`          |                |                                 |

## BrowserServer

| Status | Playweight                  | WebDriver Bidi | Note                         |
| ------ | --------------------------- | -------------- | ---------------------------- |
| ❌     | `BrowserServer.close`       | N/A            | Child process, not protocol. |
| ❌     | `BrowserServer.kill`        | N/A            | Child process, not protocol. |
| ❌     | `BrowserServer.process`     | N/A            | Child process, not protocol. |
| ❌     | `BrowserServer.wsEndpoint`  | N/A            | Child process, not protocol. |
| ❌     | `BrowserServer.on('close')` | N/A            | Child process, not protocol. |

## BrowserType

| Status | Playweight                            | WebDriver Bidi | Note                            |
| ------ | ------------------------------------- | -------------- | ------------------------------- |
| ❌     | `BrowserType.connect`                 | N/A            | Establish WebSocket connection. |
| 🚫     | `BrowserType.connectOverCDP`          | N/A            | We use WebDriver Bidi, not CDP. |
| ❌     | `BrowserType.executablePath`          | N/A            | Establish WebSocket connection. |
| ✅     | `BrowserType.launch`                  | N/A            | Child process and connect.      |
| ❔     | `BrowserType.launchPersistentContext` |                |                                 |
| ❌     | `BrowserType.launchServer`            | N/A            | Child process, not protocol.    |
| ✅     | `BrowserType.name`                    | N/A            | Implementation.                 |

## CDPSession

| Status | Playweight          | WebDriver Bidi | Note                            |
| ------ | ------------------- | -------------- | ------------------------------- |
| 🚫     | `CDPSession.detach` | N/A            | We use WebDriver Bidi, not CDP. |
| 🚫     | `CDPSession.send`   | N/A            | We use WebDriver Bidi, not CDP. |

## Clock

> [!NOTE]
>
> Clock seems to be implemented by injecting `setTimeout` functions to override.

| Status | Playweight            | WebDriver Bidi | Note |
| ------ | --------------------- | -------------- | ---- |
| ❔     | `Clock.fastForward`   | N/A            |      |
| ❔     | `Clock.install`       | N/A            |      |
| ❔     | `Clock.pauseAt`       | N/A            |      |
| ❔     | `Clock.resume`        | N/A            |      |
| ❔     | `Clock.runFor`        | N/A            |      |
| ❔     | `Clock.setFixedtime`  | N/A            |      |
| ❔     | `Clock.setSystemTime` | N/A            |      |

## ConsoleMessage

| Status | Playweight                | WebDriver Bidi            | Note |
| ------ | ------------------------- | ------------------------- | ---- |
| ❌     | `ConsoleMessage.args`     | `log.LogEntry` properties |      |
| ❌     | `ConsoleMessage.location` | `log.LogEntry` properties |      |
| ❌     | `ConsoleMessage.page`     | `log.LogEntry` properties |      |
| ❌     | `ConsoleMessage.text`     | `log.LogEntry` properties |      |
| ❌     | `ConsoleMessage.type`     | `log.LogEntry` properties |      |

## Coverage

| Status | Playweight                  | WebDriver Bidi | Note          |
| ------ | --------------------------- | -------------- | ------------- |
| 🚫     | `Coverage.startCSSCoverage` |                | CDP Specific. |
| 🚫     | `Coverage.startJSCoverage`  |                | CDP Specific. |
| 🚫     | `Coverage.stopCSSCoverage`  |                | CDP Specific. |
| 🚫     | `Coverage.stopJSCoverage`   |                | CDP Specific. |

## Dialog

| Status | Playweight            | WebDriver Bidi                                          | Note |
| ------ | --------------------- | ------------------------------------------------------- | ---- |
| ❌     | `Dialog.accept`       | `browsingContext.UserPromptOpenedParameters` properties |      |
| ❌     | `Dialog.defaultValue` | `browsingContext.UserPromptOpenedParameters` properties |      |
| ❌     | `Dialog.dismiss`      | `browsingContext.UserPromptOpenedParameters` properties |      |
| ❌     | `Dialog.message`      | `browsingContext.UserPromptOpenedParameters` properties |      |
| ❌     | `Dialog.page`         | `browsingContext.UserPromptOpenedParameters` properties |      |
| ❌     | `Dialog.type`         | `browsingContext.UserPromptOpenedParameters` properties |      |

## Download

| Status | Playweight                   | WebDriver Bidi                                 | Note |
| ------ | ---------------------------- | ---------------------------------------------- | ---- |
| ❌     | `Download.cancel`            | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.createReadStream`  | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.delete`            | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.failure`           | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.page`              | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.path`              | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.saveAs`            | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.suggestedFilename` | `browsingContext.DownloadEndParams` properties |      |
| ❌     | `Download.url`               | `browsingContext.DownloadEndParams` properties |      |

## ElementHandle

| Status | Playweight                             | WebDriver Bidi | Note |
| ------ | -------------------------------------- | -------------- | ---- |
| ❔     | `ElementHandle.boundingBox`            |                |      |
| ❔     | `ElementHandle.contentFrame`           |                |      |
| ❔     | `ElementHandle.ownerFrame`             |                |      |
| ❔     | `ElementHandle.waitForElementState`    |                |      |
| ❔     | `ElementHandle.$`                      |                |      |
| ❔     | `ElementHandle.$$`                     |                |      |
| ❔     | `ElementHandle.$eval`                  |                |      |
| ❔     | `ElementHandle.$$eval`                 |                |      |
| ❔     | `ElementHandle.check`                  |                |      |
| ❔     | `ElementHandle.click`                  |                |      |
| ❔     | `ElementHandle.dblclick`               |                |      |
| ❔     | `ElementHandle.dispatchEvent`          |                |      |
| ❔     | `ElementHandle.fill`                   |                |      |
| ❔     | `ElementHandle.focus`                  |                |      |
| ❔     | `ElementHandle.getAttribute`           |                |      |
| ❔     | `ElementHandle.hover`                  |                |      |
| ❔     | `ElementHandle.innerHTML`              |                |      |
| ❔     | `ElementHandle.innerText`              |                |      |
| ❔     | `ElementHandle.inputValue`             |                |      |
| ❔     | `ElementHandle.isChecked`              |                |      |
| ❔     | `ElementHandle.isDisabled`             |                |      |
| ❔     | `ElementHandle.isEditable`             |                |      |
| ❔     | `ElementHandle.isEnabled`              |                |      |
| ❔     | `ElementHandle.isHidden`               |                |      |
| ❔     | `ElementHandle.isVisible`              |                |      |
| ❔     | `ElementHandle.press`                  |                |      |
| ❔     | `ElementHandle.screenshot`             |                |      |
| ❔     | `ElementHandle.scrollIntoViewIfNeeded` |                |      |
| ❔     | `ElementHandle.selectOption`           |                |      |
| ❔     | `ElementHandle.selectText`             |                |      |
| ❔     | `ElementHandle.setChecked`             |                |      |
| ❔     | `ElementHandle.setInputFiles`          |                |      |
| ❔     | `ElementHandle.tap`                    |                |      |
| ❔     | `ElementHandle.textContent`            |                |      |
| ❔     | `ElementHandle.type`                   |                |      |
| ❔     | `ElementHandle.uncheck`                |                |      |
| ❔     | `ElementHandle.waitForSelector`        |                |      |

## FileChooser

| Status | Playweight               | WebDriver Bidi                    | Note |
| ------ | ------------------------ | --------------------------------- | ---- |
| ❌     | `FileChooser.element`    | `input.fileDialogInfo` properties |      |
| ❌     | `FileChooser.isMultiple` | `input.fileDialogInfo` properties |      |
| ❌     | `FileChooser.page`       | `input.fileDialogInfo` properties |      |
| ❌     | `FileChooser.setFiles`   | `input.fileDialogInfo` properties |      |

## Frame

| Status | Playweight                | WebDriver Bidi | Note |
| ------ | ------------------------- | -------------- | ---- |
| ❔     | `Frame.addScriptTag`      |                |      |
| ❔     | `Frame.addStyleTag`       |                |      |
| ❔     | `Frame.childFrames`       |                |      |
| ❔     | `Frame.content`           |                |      |
| ❔     | `Frame.dragAndDrop`       |                |      |
| ❔     | `Frame.evaluate`          |                |      |
| ❔     | `Frame.evaluateHandle`    |                |      |
| ❔     | `Frame.frameElement`      |                |      |
| ❔     | `Frame.frameLocator`      |                |      |
| ❔     | `Frame.getByAltText`      |                |      |
| ❔     | `Frame.getByLabel`        |                |      |
| ❔     | `Frame.getByPlaceholder`  |                |      |
| ❔     | `Frame.getByRole`         |                |      |
| ❔     | `Frame.getByTestId`       |                |      |
| ❔     | `Frame.getByText`         |                |      |
| ❔     | `Frame.getByTitle`        |                |      |
| ❔     | `Frame.goto`              |                |      |
| ❔     | `Frame.isDetached`        |                |      |
| ❔     | `Frame.isEnabled`         |                |      |
| ❔     | `Frame.locator`           |                |      |
| ❔     | `Frame.name`              |                |      |
| ❔     | `Frame.page`              |                |      |
| ❔     | `Frame.parentFrame`       |                |      |
| ❔     | `Frame.setContent`        |                |      |
| ❔     | `Frame.title`             |                |      |
| ❔     | `Frame.url`               |                |      |
| ❔     | `Frame.waitForFunction`   |                |      |
| ❔     | `Frame.waitForLoadState`  |                |      |
| ❔     | `Frame.waitForURL`        |                |      |
| ❔     | `Frame.$`                 |                |      |
| ❔     | `Frame.$$`                |                |      |
| ❔     | `Frame.$eval`             |                |      |
| ❔     | `Frame.$$eval`            |                |      |
| ❔     | `Frame.check`             |                |      |
| ❔     | `Frame.click`             |                |      |
| ❔     | `Frame.dblclick`          |                |      |
| ❔     | `Frame.dispatchEvent`     |                |      |
| ❔     | `Frame.fill`              |                |      |
| ❔     | `Frame.focus`             |                |      |
| ❔     | `Frame.getAttribute`      |                |      |
| ❔     | `Frame.hover`             |                |      |
| ❔     | `Frame.innerHTML`         |                |      |
| ❔     | `Frame.innerText`         |                |      |
| ❔     | `Frame.inputValue`        |                |      |
| ❔     | `Frame.isChecked`         |                |      |
| ❔     | `Frame.isDisabled`        |                |      |
| ❔     | `Frame.isEditable`        |                |      |
| ❔     | `Frame.isHidden`          |                |      |
| ❔     | `Frame.isVisible`         |                |      |
| ❔     | `Frame.press`             |                |      |
| ❔     | `Frame.selectOption`      |                |      |
| ❔     | `Frame.setChecked`        |                |      |
| ❔     | `Frame.setInputFiles`     |                |      |
| ❔     | `Frame.tap`               |                |      |
| ❔     | `Frame.textContent`       |                |      |
| ❔     | `Frame.type`              |                |      |
| ❔     | `Frame.uncheck`           |                |      |
| ❔     | `Frame.waitForNavigation` |                |      |
| ❔     | `Frame.waitForSelector`   |                |      |
| ❔     | `Frame.waitForTimeout`    |                |      |

## FrameLocator

| Status | Playweight                      | WebDriver Bidi | Note |
| ------ | ------------------------------- | -------------- | ---- |
| ❔     | `FrameLocator.frameLocator`     |                |      |
| ❔     | `FrameLocator.getByAltText`     |                |      |
| ❔     | `FrameLocator.getByLabel`       |                |      |
| ❔     | `FrameLocator.getByPlaceholder` |                |      |
| ❔     | `FrameLocator.getByRole`        |                |      |
| ❔     | `FrameLocator.getByTestId`      |                |      |
| ❔     | `FrameLocator.getByText`        |                |      |
| ❔     | `FrameLocator.getByTitle`       |                |      |
| ❔     | `FrameLocator.locator`          |                |      |
| ❔     | `FrameLocator.owner`            |                |      |
| ❔     | `FrameLocator.first`            |                |      |
| ❔     | `FrameLocator.last`             |                |      |
| ❔     | `FrameLocator.nth`              |                |      |

## JSHandle

| Status | Playweight                | WebDriver Bidi | Note |
| ------ | ------------------------- | -------------- | ---- |
| ❔     | `JSHandle.asElement`      |                |      |
| ❔     | `JSHandle.dispose`        |                |      |
| ❔     | `JSHandle.evaluate`       |                |      |
| ❔     | `JSHandle.evaluateHandle` |                |      |
| ❔     | `JSHandle.getProperties`  |                |      |
| ❔     | `JSHandle.getProperty`    |                |      |
| ❔     | `JSHandle.jsonValue`      |                |      |

## Keyboard

| Status | Playweight                | WebDriver Bidi        | Note |
| ------ | ------------------------- | --------------------- | ---- |
| ❌     | `Keyboard.asElement`      | `input.performAction` |      |
| ❌     | `Keyboard.dispose`        | `input.performAction` |      |
| ❌     | `Keyboard.evaluate`       | `input.performAction` |      |
| ❌     | `Keyboard.evaluateHandle` | `input.performAction` |      |
| ❌     | `Keyboard.getProperties`  | `input.performAction` |      |
| ❌     | `Keyboard.getProperty`    | `input.performAction` |      |
| ❌     | `Keyboard.jsonValue`      | `input.performAction` |      |

## Locator

| Status | Playweight                       | WebDriver Bidi | Note |
| ------ | -------------------------------- | -------------- | ---- |
| ❔     | `Locator.all`                    |                |      |
| ❔     | `Locator.allInnerTexts`          |                |      |
| ❔     | `Locator.allTextContents`        |                |      |
| ❔     | `Locator.and`                    |                |      |
| ❔     | `Locator.ariaSnapshot`           |                |      |
| ❔     | `Locator.blur`                   |                |      |
| ❔     | `Locator.boundingBox`            |                |      |
| ❔     | `Locator.check`                  |                |      |
| ❔     | `Locator.clear`                  |                |      |
| ❔     | `Locator.click`                  |                |      |
| ❔     | `Locator.contentFrame`           |                |      |
| ❔     | `Locator.count`                  |                |      |
| ❔     | `Locator.dblclick`               |                |      |
| ❔     | `Locator.describe`               |                |      |
| ❔     | `Locator.dispatchEvent`          |                |      |
| ❔     | `Locator.dragTo`                 |                |      |
| ❔     | `Locator.evaluate`               |                |      |
| ❔     | `Locator.evaluateAll`            |                |      |
| ❔     | `Locator.evaluateHandle`         |                |      |
| ❔     | `Locator.fill`                   |                |      |
| ❔     | `Locator.filter`                 |                |      |
| ❔     | `Locator.first`                  |                |      |
| ❔     | `Locator.focus`                  |                |      |
| ❔     | `Locator.frameLocator`           |                |      |
| ❔     | `Locator.getAttribute`           |                |      |
| ❔     | `Locator.getByAltText`           |                |      |
| ❔     | `Locator.getByLabel`             |                |      |
| ❔     | `Locator.getByPlaceholder`       |                |      |
| ❔     | `Locator.getByRole`              |                |      |
| ❔     | `Locator.getByTestId`            |                |      |
| ❔     | `Locator.getByText`              |                |      |
| ❔     | `Locator.getByTitle`             |                |      |
| ❔     | `Locator.highlight`              |                |      |
| ❔     | `Locator.hover`                  |                |      |
| ❔     | `Locator.innerHTML`              |                |      |
| ❔     | `Locator.innerText`              |                |      |
| ❔     | `Locator.inputValue`             |                |      |
| ❔     | `Locator.isChecked`              |                |      |
| ❔     | `Locator.isDisabled`             |                |      |
| ❔     | `Locator.isEditable`             |                |      |
| ❔     | `Locator.isEnabled`              |                |      |
| ❔     | `Locator.isHidden`               |                |      |
| ❔     | `Locator.isVisible`              |                |      |
| ❔     | `Locator.last`                   |                |      |
| ❔     | `Locator.locator`                |                |      |
| ❔     | `Locator.nth`                    |                |      |
| ❔     | `Locator.or`                     |                |      |
| ❔     | `Locator.page`                   |                |      |
| ❔     | `Locator.press`                  |                |      |
| ❔     | `Locator.pressSequentially`      |                |      |
| ❔     | `Locator.screenshot`             |                |      |
| ❔     | `Locator.scrollIntoViewIfNeeded` |                |      |
| ❔     | `Locator.selectOption`           |                |      |
| ❔     | `Locator.selectText`             |                |      |
| ❔     | `Locator.setChecked`             |                |      |
| ❔     | `Locator.setInputFiles`          |                |      |
| ❔     | `Locator.tap`                    |                |      |
| ❔     | `Locator.textContent`            |                |      |
| ❔     | `Locator.uncheck`                |                |      |
| ❔     | `Locator.waitFor`                |                |      |
| ❔     | `Locator.elementHandle`          |                |      |
| ❔     | `Locator.elementHandles`         |                |      |
| ❔     | `Locator.type`                   |                |      |

## Logger

| Status | Playweight         | WebDriver Bidi | Note |
| ------ | ------------------ | -------------- | ---- |
| ❔     | `Logger.isEnabled` |                |      |
| ❔     | `Logger.log`       |                |      |

## Mouse

| Status | Playweight        | WebDriver Bidi        | Note |
| ------ | ----------------- | --------------------- | ---- |
| ❌     | ` Mouse.click`    | `input.performAction` |      |
| ❌     | ` Mouse.dblclick` | `input.performAction` |      |
| ❌     | ` Mouse.down`     | `input.performAction` |      |
| ❌     | ` Mouse.move`     | `input.performAction` |      |
| ❌     | ` Mouse.up`       | `input.performAction` |      |
| ❌     | ` Mouse.wheel`    | `input.performAction` |      |

## Page

| Status | Playweight                         | WebDriver Bidi | Note |
| ------ | ---------------------------------- | -------------- | ---- |
| ❔     | ` Page.addInitScript`              |                |      |
| ❔     | ` Page.addLocatorHandler`          |                |      |
| ❔     | ` Page.addScriptTag`               |                |      |
| ❔     | ` Page.addStyleTag`                |                |      |
| ❔     | ` Page.bringToFront`               |                |      |
| ❔     | ` Page.close`                      |                |      |
| ❔     | ` Page.consoleMessages`            |                |      |
| ❔     | ` Page.content`                    |                |      |
| ❔     | ` Page.context`                    |                |      |
| ❔     | ` Page.dragAndDrop`                |                |      |
| ❔     | ` Page.emulateMedia`               |                |      |
| ❔     | ` Page.evaluate`                   |                |      |
| ❔     | ` Page.evaluateHandle`             |                |      |
| ❔     | ` Page.exposeBinding`              |                |      |
| ❔     | ` Page.exposeFunction`             |                |      |
| ❔     | ` Page.frame`                      |                |      |
| ❔     | ` Page.frameLocator`               |                |      |
| ❔     | ` Page.frames`                     |                |      |
| ❔     | ` Page.getByAltText`               |                |      |
| ❔     | ` Page.getByLabel`                 |                |      |
| ❔     | ` Page.getByPlaceholder`           |                |      |
| ❔     | ` Page.getByRole`                  |                |      |
| ❔     | ` Page.getByTestId`                |                |      |
| ❔     | ` Page.getByText`                  |                |      |
| ❔     | ` Page.getByTitle`                 |                |      |
| ❔     | ` Page.goBack`                     |                |      |
| ❔     | ` Page.goForward`                  |                |      |
| ❔     | ` Page.goto`                       |                |      |
| ❔     | ` Page.isClosed`                   |                |      |
| ❔     | ` Page.locator`                    |                |      |
| ❔     | ` Page.mainFrame`                  |                |      |
| ❔     | ` Page.opener`                     |                |      |
| ❔     | ` Page.pageErrors`                 |                |      |
| ❔     | ` Page.pause`                      |                |      |
| ❔     | ` Page.pdf`                        |                |      |
| ❔     | ` Page.reload`                     |                |      |
| ❔     | ` Page.removeAllListeners`         |                |      |
| ❔     | ` Page.removeLocatorHandler`       |                |      |
| ❔     | ` Page.requestGC`                  |                |      |
| ❔     | ` Page.requests`                   |                |      |
| ❔     | ` Page.route`                      |                |      |
| ❔     | ` Page.routeFromHAR`               |                |      |
| ❔     | ` Page.routeWebSocket`             |                |      |
| ❔     | ` Page.screenshot`                 |                |      |
| ❔     | ` Page.setContent`                 |                |      |
| ❔     | `Page.setDefaultNavigationTimeout` |                |      |
| ❔     | `Page.setDefaultTimeout`           |                |      |
| ❔     | `Page.setExtraHTTPHeaders`         |                |      |
| ❔     | `Page.setViewportSize`             |                |      |
| ❔     | `Page.title`                       |                |      |
| ❔     | `Page.unroute`                     |                |      |
| ❔     | `Page.unrouteAll`                  |                |      |
| ❔     | `Page.url`                         |                |      |
| ❔     | `Page.video`                       |                |      |
| ❔     | `Page.viewportSize`                |                |      |
| ❔     | `Page.waitForEvent`                |                |      |
| ❔     | `Page.waitForFunction`             |                |      |
| ❔     | `Page.waitForLoadState`            |                |      |
| ❔     | `Page.waitForRequest`              |                |      |
| ❔     | `Page.waitForResponse`             |                |      |
| ❔     | `Page.waitForURL`                  |                |      |
| ❔     | `Page.workers`                     |                |      |
| ❔     | `Page.Properties`                  |                |      |
| ❔     | `Page.clock`                       |                |      |
| ❔     | `Page.coverage`                    |                |      |
| ❔     | `Page.keyboard`                    |                |      |
| ❔     | `Page.mouse`                       |                |      |
| ❔     | `Page.request`                     |                |      |
| ❔     | `Page.touchscreen`                 |                |      |
| ❔     | `Page.on('close')`                 |                |      |
| ❔     | `Page.on('console')`               |                |      |
| ❔     | `Page.on('crash')`                 |                |      |
| ❔     | `Page.on('dialog')`                |                |      |
| ❔     | `Page.on('domcontentloaded')`      |                |      |
| ❔     | `Page.on('download')`              |                |      |
| ❔     | `Page.on('filechooser')`           |                |      |
| ❔     | `Page.on('frameattached')`         |                |      |
| ❔     | `Page.on('framedetached')`         |                |      |
| ❔     | `Page.on('framenavigated')`        |                |      |
| ❔     | `Page.on('load')`                  |                |      |
| ❔     | `Page.on('pageerror')`             |                |      |
| ❔     | `Page.on('popup')`                 |                |      |
| ❔     | `Page.on('request')`               |                |      |
| ❔     | `Page.on('requestfailed')`         |                |      |
| ❔     | `Page.on('requestfinished')`       |                |      |
| ❔     | `Page.on('response')`              |                |      |
| ❔     | `Page.on('websocket')`             |                |      |
| ❔     | `Page.on('worker')`                |                |      |
| ❔     | `Page.$`                           |                |      |
| ❔     | `Page.$$`                          |                |      |
| ❔     | `Page.$eval`                       |                |      |
| ❔     | `Page.$$eval`                      |                |      |
| ❔     | `Page.accessibility`               |                |      |
| ❔     | `Page.check`                       |                |      |
| ❔     | `Page.click`                       |                |      |
| ❔     | `Page.dblclick`                    |                |      |
| ❔     | `Page.dispatchEvent`               |                |      |
| ❔     | `Page.fill`                        |                |      |
| ❔     | `Page.focus`                       |                |      |
| ❔     | `Page.getAttribute`                |                |      |
| ❔     | `Page.hover`                       |                |      |
| ❔     | `Page.innerHTML`                   |                |      |
| ❔     | `Page.innerText`                   |                |      |
| ❔     | `Page.inputValue`                  |                |      |
| ❔     | `Page.isChecked`                   |                |      |
| ❔     | `Page.isDisabled`                  |                |      |
| ❔     | `Page.isEditable`                  |                |      |
| ❔     | `Page.isEnabled`                   |                |      |
| ❔     | `Page.isHidden`                    |                |      |
| ❔     | `Page.isVisible`                   |                |      |
| ❔     | `Page.press`                       |                |      |
| ❔     | `Page.selectOption`                |                |      |
| ❔     | `Page.setChecked`                  |                |      |
| ❔     | `Page.setInputFiles`               |                |      |
| ❔     | `Page.tap`                         |                |      |
| ❔     | `Page.textContent`                 |                |      |
| ❔     | `Page.type`                        |                |      |
| ❔     | `Page.uncheck`                     |                |      |
| ❔     | `Page.waitForNavigation`           |                |      |
| ❔     | `Page.waitForSelector`             |                |      |
| ❔     | `Page.waitForTimeout`              |                |      |

## Request

| Status | Playweight                    | WebDriver Bidi | Note |
| ------ | ----------------------------- | -------------- | ---- |
| ❔     | `Request.allHeaders`          |                |      |
| ❔     | `Request.failure`             |                |      |
| ❔     | `Request.frame`               |                |      |
| ❔     | `Request.headerValue`         |                |      |
| ❔     | `Request.headers`             |                |      |
| ❔     | `Request.headersArray`        |                |      |
| ❔     | `Request.isNavigationRequest` |                |      |
| ❔     | `Request.method`              |                |      |
| ❔     | `Request.postData`            |                |      |
| ❔     | `Request.postDataBuffer`      |                |      |
| ❔     | `Request.postDataJSON`        |                |      |
| ❔     | `Request.redirectedFrom`      |                |      |
| ❔     | `Request.redirectedTo`        |                |      |
| ❔     | `Request.resourceType`        |                |      |
| ❔     | `Request.response`            |                |      |
| ❔     | `Request.serviceWorker`       |                |      |
| ❔     | `Request.sizes`               |                |      |
| ❔     | `Request.timing`              |                |      |
| ❔     | `Request.url`                 |                |      |

## Response

| Status | Playweight                     | WebDriver Bidi | Note |
| ------ | ------------------------------ | -------------- | ---- |
| ❔     | `Response.allHeaders`          |                |      |
| ❔     | `Response.failure`             |                |      |
| ❔     | `Response.frame`               |                |      |
| ❔     | `Response.headerValue`         |                |      |
| ❔     | `Response.headers`             |                |      |
| ❔     | `Response.headersArray`        |                |      |
| ❔     | `Response.isNavigationRequest` |                |      |
| ❔     | `Response.method`              |                |      |
| ❔     | `Response.postData`            |                |      |
| ❔     | `Response.postDataBuffer`      |                |      |
| ❔     | `Response.postDataJSON`        |                |      |
| ❔     | `Response.redirectedFrom`      |                |      |
| ❔     | `Response.redirectedTo`        |                |      |
| ❔     | `Response.resourceType`        |                |      |
| ❔     | `Response.response`            |                |      |
| ❔     | `Response.serviceWorker`       |                |      |
| ❔     | `Response.sizes`               |                |      |
| ❔     | `Response.timing`              |                |      |
| ❔     | `Response.url`                 |                |      |

## Route

| Status | Playweight       | WebDriver Bidi | Note |
| ------ | ---------------- | -------------- | ---- |
| ❔     | `Route.abort`    |                |      |
| ❔     | `Route.continue` |                |      |
| ❔     | `Route.fallback` |                |      |
| ❔     | `Route.fetch`    |                |      |
| ❔     | `Route.fulfill`  |                |      |
| ❔     | `Route.request`  |                |      |

## Selectors

| Status | Playweight                     | WebDriver Bidi | Note |
| ------ | ------------------------------ | -------------- | ---- |
| ❔     | `Selectors.register`           |                |      |
| ❔     | `Selectors.setTestIdAttribute` |                |      |

## Touchscreen

| Status | Playweight        | WebDriver Bidi | Note |
| ------ | ----------------- | -------------- | ---- |
| ❔     | `Touchscreen.tap` |                |      |

## Tracing

| Status | Playweight           | WebDriver Bidi | Note |
| ------ | -------------------- | -------------- | ---- |
| 🚫     | `Tracing.group`      |                |      |
| 🚫     | `Tracing.groupEnd`   |                |      |
| 🚫     | `Tracing.start`      |                |      |
| 🚫     | `Tracing.startChunk` |                |      |
| 🚫     | `Tracing.stop`       |                |      |
| 🚫     | `Tracing.stopChunk`  |                |      |

## Video

| Status | Playweight     | WebDriver Bidi | Note |
| ------ | -------------- | -------------- | ---- |
| ❔     | `Video.delete` |                |      |
| ❔     | `Video.path`   |                |      |
| ❔     | `Video.saveAs` |                |      |

## WebSocket

| Status | Playweight                      | WebDriver Bidi | Note |
| ------ | ------------------------------- | -------------- | ---- |
| ❔     | `WebSocket.isClosed`            |                |      |
| ❔     | `WebSocket.url`                 |                |      |
| ❔     | `WebSocket.waitForEvent`        |                |      |
| ❔     | `WebSocket.on('close')`         |                |      |
| ❔     | `WebSocket.on('framereceived')` |                |      |
| ❔     | `WebSocket.on('framesent')`     |                |      |
| ❔     | `WebSocket.on('socketerror')`   |                |      |

## WebSocketRoute

| Status | Playweight                       | WebDriver Bidi | Note |
| ------ | -------------------------------- | -------------- | ---- |
| ❔     | `WebSocketRoute.close`           |                |      |
| ❔     | `WebSocketRoute.connectToServer` |                |      |
| ❔     | `WebSocketRoute.onClose`         |                |      |
| ❔     | `WebSocketRoute.onMessage`       |                |      |
| ❔     | `WebSocketRoute.send`            |                |      |
| ❔     | `WebSocketRoute.url`             |                |      |

## Worker

| Status | Playweight              | WebDriver Bidi | Note |
| ------ | ----------------------- | -------------- | ---- |
| ❔     | `Worker.evaluate`       |                |      |
| ❔     | `Worker.evaluateHandle` |                |      |
| ❔     | `Worker.url`            |                |      |
| ❔     | `Worker.on('close')`    |                |      |
