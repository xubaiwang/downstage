# Mapping from Playwright

Categorized by classes:

- APIRequest
- APIRequestContext
- APIResponse
- Accessibility
- Browser
- BrowserContext
- BrowserServer
- BrowserType
- CDPSession
- Clock
- ConsoleMessage
- Coverage
- Dialog
- Download
- ElementHandle
- FileChooser
- Frame
- FrameLocator
- JSHandle
- Keyboard
- Locator
- Logger
- Mouse
- Page
- Request
- Response
- Route
- Selectors
- Touchscreen
- Tracing
- Video
- WebError
- WebSocket
- WebSocketRoute
- Worker

## APIRequest

| Playwright                | WebDriver Bidi | Note |
| ------------------------- | -------------- | ---- |
| `APIRequest.newContext()` |                |      |

## APIRequestContext

| Playwright                        | WebDriver Bidi | Note |
| --------------------------------- | -------------- | ---- |
| `APIRequesContext.delete()`       |                |      |
| `APIRequesContext.dispose()`      |                |      |
| `APIRequesContext.fetch()`        |                |      |
| `APIRequesContext.get()`          |                |      |
| `APIRequesContext.head()`         |                |      |
| `APIRequesContext.patch()`        |                |      |
| `APIRequesContext.post()`         |                |      |
| `APIRequesContext.put()`          |                |      |
| `APIRequesContext.storageState()` |                |      |

## APIResponse

| Playweight                 | WebDriver Bidi | Note |
| -------------------------- | -------------- | ---- |
| `APIResponse.body`         |                |      |
| `APIResponse.dispose`      |                |      |
| `APIResponse.headers`      |                |      |
| `APIResponse.headersArray` |                |      |
| `APIResponse.json`         |                |      |
| `APIResponse.ok`           |                |      |
| `APIResponse.status`       |                |      |
| `APIResponse.statusText`   |                |      |
| `APIResponse.text`         |                |      |
| `APIResponse.url`          |                |      |

## Accessibility

> [!NOTE]
>
> Deprecated, lower precedence

| Playweight               | WebDriver Bidi | Note |
| ------------------------ | -------------- | ---- |
| `Accessibility.snapshot` |                |      |

## Browser

| Status | Playweight                     | WebDriver Bidi | Note                            |
| ------ | ------------------------------ | -------------- | ------------------------------- |
|        | `Browser.browserType`          |                |                                 |
|        | `Browser.close`                |                |                                 |
|        | `Browser.contexts`             |                |                                 |
|        | `Browser.isConnected`          |                |                                 |
| 🚫     | `Browser.newBrowserCDPSession` | N/A            | We use WebDriver Bidi, not CDP. |
|        | `Browser.newContext`           |                |                                 |
|        | `Browser.newPage`              |                |                                 |
|        | `Browser.removeAllListeners`   |                |                                 |
|        | `Browser.startTracing`         |                |                                 |
|        | `Browser.stopTracing`          |                |                                 |
|        | `Browser.version`              |                |                                 |
|        | `Browser.on('disconnected')`   |                |                                 |

## BrowserContext

| Status | Playweight                            | WebDriver Bidi | Note                            |
| ------ | ------------------------------------- | -------------- | ------------------------------- |
|        | `Browser.addCookies`                  |                |                                 |
|        | `Browser.addInitScript`               |                |                                 |
|        | `Browser.browser`                     |                |                                 |
|        | `Browser.clearCookies`                |                |                                 |
|        | `Browser.clearPermissions`            |                |                                 |
|        | `Browser.close`                       |                |                                 |
|        | `Browser.cookies`                     |                |                                 |
|        | `Browser.exposeBinding`               |                |                                 |
|        | `Browser.exposeFunction`              |                |                                 |
|        | `Browser.grantPermissions`            |                |                                 |
| 🚫     | `Browser.newCDPSession`               |                | We use WebDriver Bidi, not CDP. |
|        | `Browser.newPage`                     |                |                                 |
|        | `Browser.pages`                       |                |                                 |
|        | `Browser.removeAllListeners`          |                |                                 |
|        | `Browser.route`                       |                |                                 |
|        | `Browser.routeFromHAR`                |                |                                 |
|        | `Browser.routeWebSocket`              |                |                                 |
|        | `Browser.serviceWorkers`              |                |                                 |
|        | `Browser.setDefaultNavigationTimeout` |                |                                 |
|        | `Browser.setDefaultTimeout`           |                |                                 |
|        | `Browser.setExtraHTTPHeaders`         |                |                                 |
|        | `Browser.setGeolocation`              |                |                                 |
|        | `Browser.setOffline`                  |                |                                 |
|        | `Browser.storageState`                |                |                                 |
|        | `Browser.unroute`                     |                |                                 |
|        | `Browser.unrouteAll`                  |                |                                 |
|        | `Browser.waitForEvent`                |                |                                 |
|        | `Browser.clock`                       |                |                                 |
|        | `Browser.request`                     |                |                                 |
|        | `Browser.tracing`                     |                |                                 |
|        | `Browser.on('close')`                 |                |                                 |
|        | `Browser.on('console')`               |                |                                 |
|        | `Browser.on('dialog')`                |                |                                 |
|        | `Browser.on('page')`                  |                |                                 |
|        | `Browser.on('request')`               |                |                                 |
|        | `Browser.on('requestfailed')`         |                |                                 |
|        | `Browser.on('requestfinished')`       |                |                                 |
|        | `Browser.on('response')`              |                |                                 |
|        | `Browser.on('serviceworker')`         |                |                                 |
|        | `Browser.on('weberror')`              |                |                                 |
|        | `Browser.on('backgroundpage')`        |                |                                 |
|        | `Browser.backgroundPages`             |                |                                 |
|        | `Browser.setHTTPCredentials`          |                |                                 |

## BrowserServer

| Playweight                  | WebDriver Bidi | Note |
| --------------------------- | -------------- | ---- |
| `BrowserServer.close`       |                |      |
| `BrowserServer.kill`        |                |      |
| `BrowserServer.process`     |                |      |
| `BrowserServer.wsEndpoint`  |                |      |
| `BrowserServer.on('close')` |                |      |

## BrowserType

|     | Playweight                            | WebDriver Bidi | Note                            |
| --- | ------------------------------------- | -------------- | ------------------------------- |
|     | `BrowserType.connect`                 |                |                                 |
| 🚫  | `BrowserType.connectOverCDP`          |                | We use WebDriver Bidi, not CDP. |
|     | `BrowserType.executablePath`          |                |                                 |
|     | `BrowserType.launch`                  |                |                                 |
|     | `BrowserType.launchPersistentContext` |                |                                 |
|     | `BrowserType.launchServer`            |                |                                 |
|     | `BrowserType.name`                    |                |                                 |

## CDPSession

|     | Playweight          | WebDriver Bidi | Note                            |
| --- | ------------------- | -------------- | ------------------------------- |
| 🚫  | `CDPSession.detach` |                | We use WebDriver Bidi, not CDP. |
| 🚫  | `CDPSession.send`   |                | We use WebDriver Bidi, not CDP. |

## Clock

| Playweight            | WebDriver Bidi | Note |
| --------------------- | -------------- | ---- |
| `Clock.fastForward`   |                |      |
| `Clock.install`       |                |      |
| `Clock.pauseAt`       |                |      |
| `Clock.resume`        |                |      |
| `Clock.runFor`        |                |      |
| `Clock.setFixedtime`  |                |      |
| `Clock.setSystemTime` |                |      |

## ConsoleMessage

| Playweight                | WebDriver Bidi | Note |
| ------------------------- | -------------- | ---- |
| `ConsoleMessage.args`     |                |      |
| `ConsoleMessage.location` |                |      |
| `ConsoleMessage.page`     |                |      |
| `ConsoleMessage.text`     |                |      |
| `ConsoleMessage.type`     |                |      |

## Coverage

| Playweight                  | WebDriver Bidi | Note |
| --------------------------- | -------------- | ---- |
| `Coverage.startCSSCoverage` |                |      |
| `Coverage.startJSCoverage`  |                |      |
| `Coverage.stopCSSCoverage`  |                |      |
| `Coverage.stopJSCoverage`   |                |      |

## Dialog

| Playweight            | WebDriver Bidi | Note |
| --------------------- | -------------- | ---- |
| `Dialog.accept`       |                |      |
| `Dialog.defaultValue` |                |      |
| `Dialog.dismiss`      |                |      |
| `Dialog.message`      |                |      |
| `Dialog.page`         |                |      |
| `Dialog.type`         |                |      |

## Download

| Playweight                   | WebDriver Bidi | Note |
| ---------------------------- | -------------- | ---- |
| `Download.cancel`            |                |      |
| `Download.createReadStream`  |                |      |
| `Download.delete`            |                |      |
| `Download.failure`           |                |      |
| `Download.page`              |                |      |
| `Download.path`              |                |      |
| `Download.saveAs`            |                |      |
| `Download.suggestedFilename` |                |      |
| `Download.url`               |                |      |

## ElementHandle

| Playweight                             | WebDriver Bidi | Note |
| -------------------------------------- | -------------- | ---- |
| `ElementHandle.boundingBox`            |                |      |
| `ElementHandle.contentFrame`           |                |      |
| `ElementHandle.ownerFrame`             |                |      |
| `ElementHandle.waitForElementState`    |                |      |
| `ElementHandle.$`                      |                |      |
| `ElementHandle.$$`                     |                |      |
| `ElementHandle.$eval`                  |                |      |
| `ElementHandle.$$eval`                 |                |      |
| `ElementHandle.check`                  |                |      |
| `ElementHandle.click`                  |                |      |
| `ElementHandle.dblclick`               |                |      |
| `ElementHandle.dispatchEvent`          |                |      |
| `ElementHandle.fill`                   |                |      |
| `ElementHandle.focus`                  |                |      |
| `ElementHandle.getAttribute`           |                |      |
| `ElementHandle.hover`                  |                |      |
| `ElementHandle.innerHTML`              |                |      |
| `ElementHandle.innerText`              |                |      |
| `ElementHandle.inputValue`             |                |      |
| `ElementHandle.isChecked`              |                |      |
| `ElementHandle.isDisabled`             |                |      |
| `ElementHandle.isEditable`             |                |      |
| `ElementHandle.isEnabled`              |                |      |
| `ElementHandle.isHidden`               |                |      |
| `ElementHandle.isVisible`              |                |      |
| `ElementHandle.press`                  |                |      |
| `ElementHandle.screenshot`             |                |      |
| `ElementHandle.scrollIntoViewIfNeeded` |                |      |
| `ElementHandle.selectOption`           |                |      |
| `ElementHandle.selectText`             |                |      |
| `ElementHandle.setChecked`             |                |      |
| `ElementHandle.setInputFiles`          |                |      |
| `ElementHandle.tap`                    |                |      |
| `ElementHandle.textContent`            |                |      |
| `ElementHandle.type`                   |                |      |
| `ElementHandle.uncheck`                |                |      |
| `ElementHandle.waitForSelector`        |                |      |

## FileChooser

| Playweight               | WebDriver Bidi | Note |
| ------------------------ | -------------- | ---- |
| `FileChooser.element`    |                |      |
| `FileChooser.isMultiple` |                |      |
| `FileChooser.page`       |                |      |
| `FileChooser.setFiles`   |                |      |

## Frame

| Playweight                | WebDriver Bidi | Note |
| ------------------------- | -------------- | ---- |
| `Frame.addScriptTag`      |                |      |
| `Frame.addStyleTag`       |                |      |
| `Frame.childFrames`       |                |      |
| `Frame.content`           |                |      |
| `Frame.dragAndDrop`       |                |      |
| `Frame.evaluate`          |                |      |
| `Frame.evaluateHandle`    |                |      |
| `Frame.frameElement`      |                |      |
| `Frame.frameLocator`      |                |      |
| `Frame.getByAltText`      |                |      |
| `Frame.getByLabel`        |                |      |
| `Frame.getByPlaceholder`  |                |      |
| `Frame.getByRole`         |                |      |
| `Frame.getByTestId`       |                |      |
| `Frame.getByText`         |                |      |
| `Frame.getByTitle`        |                |      |
| `Frame.goto`              |                |      |
| `Frame.isDetached`        |                |      |
| `Frame.isEnabled`         |                |      |
| `Frame.locator`           |                |      |
| `Frame.name`              |                |      |
| `Frame.page`              |                |      |
| `Frame.parentFrame`       |                |      |
| `Frame.setContent`        |                |      |
| `Frame.title`             |                |      |
| `Frame.url`               |                |      |
| `Frame.waitForFunction`   |                |      |
| `Frame.waitForLoadState`  |                |      |
| `Frame.waitForURL`        |                |      |
| `Frame.$`                 |                |      |
| `Frame.$$`                |                |      |
| `Frame.$eval`             |                |      |
| `Frame.$$eval`            |                |      |
| `Frame.check`             |                |      |
| `Frame.click`             |                |      |
| `Frame.dblclick`          |                |      |
| `Frame.dispatchEvent`     |                |      |
| `Frame.fill`              |                |      |
| `Frame.focus`             |                |      |
| `Frame.getAttribute`      |                |      |
| `Frame.hover`             |                |      |
| `Frame.innerHTML`         |                |      |
| `Frame.innerText`         |                |      |
| `Frame.inputValue`        |                |      |
| `Frame.isChecked`         |                |      |
| `Frame.isDisabled`        |                |      |
| `Frame.isEditable`        |                |      |
| `Frame.isHidden`          |                |      |
| `Frame.isVisible`         |                |      |
| `Frame.press`             |                |      |
| `Frame.selectOption`      |                |      |
| `Frame.setChecked`        |                |      |
| `Frame.setInputFiles`     |                |      |
| `Frame.tap`               |                |      |
| `Frame.textContent`       |                |      |
| `Frame.type`              |                |      |
| `Frame.uncheck`           |                |      |
| `Frame.waitForNavigation` |                |      |
| `Frame.waitForSelector`   |                |      |
| `Frame.waitForTimeout`    |                |      |

## FrameLocator

| Playweight                      | WebDriver Bidi | Note |
| ------------------------------- | -------------- | ---- |
| `FrameLocator.frameLocator`     |                |      |
| `FrameLocator.getByAltText`     |                |      |
| `FrameLocator.getByLabel`       |                |      |
| `FrameLocator.getByPlaceholder` |                |      |
| `FrameLocator.getByRole`        |                |      |
| `FrameLocator.getByTestId`      |                |      |
| `FrameLocator.getByText`        |                |      |
| `FrameLocator.getByTitle`       |                |      |
| `FrameLocator.locator`          |                |      |
| `FrameLocator.owner`            |                |      |
| `FrameLocator.first`            |                |      |
| `FrameLocator.last`             |                |      |
| `FrameLocator.nth`              |                |      |

## JSHandle

| Playweight                | WebDriver Bidi | Note |
| ------------------------- | -------------- | ---- |
| `JSHandle.asElement`      |                |      |
| `JSHandle.dispose`        |                |      |
| `JSHandle.evaluate`       |                |      |
| `JSHandle.evaluateHandle` |                |      |
| `JSHandle.getProperties`  |                |      |
| `JSHandle.getProperty`    |                |      |
| `JSHandle.jsonValue`      |                |      |

## Keyboard

| Playweight                | WebDriver Bidi | Note |
| ------------------------- | -------------- | ---- |
| `Keyboard.asElement`      |                |      |
| `Keyboard.dispose`        |                |      |
| `Keyboard.evaluate`       |                |      |
| `Keyboard.evaluateHandle` |                |      |
| `Keyboard.getProperties`  |                |      |
| `Keyboard.getProperty`    |                |      |
| `Keyboard.jsonValue`      |                |      |

## Locator

| Playweight                       | WebDriver Bidi | Note |
| -------------------------------- | -------------- | ---- |
| `Locator.all`                    |                |      |
| `Locator.allInnerTexts`          |                |      |
| `Locator.allTextContents`        |                |      |
| `Locator.and`                    |                |      |
| `Locator.ariaSnapshot`           |                |      |
| `Locator.blur`                   |                |      |
| `Locator.boundingBox`            |                |      |
| `Locator.check`                  |                |      |
| `Locator.clear`                  |                |      |
| `Locator.click`                  |                |      |
| `Locator.contentFrame`           |                |      |
| `Locator.count`                  |                |      |
| `Locator.dblclick`               |                |      |
| `Locator.describe`               |                |      |
| `Locator.dispatchEvent`          |                |      |
| `Locator.dragTo`                 |                |      |
| `Locator.evaluate`               |                |      |
| `Locator.evaluateAll`            |                |      |
| `Locator.evaluateHandle`         |                |      |
| `Locator.fill`                   |                |      |
| `Locator.filter`                 |                |      |
| `Locator.first`                  |                |      |
| `Locator.focus`                  |                |      |
| `Locator.frameLocator`           |                |      |
| `Locator.getAttribute`           |                |      |
| `Locator.getByAltText`           |                |      |
| `Locator.getByLabel`             |                |      |
| `Locator.getByPlaceholder`       |                |      |
| `Locator.getByRole`              |                |      |
| `Locator.getByTestId`            |                |      |
| `Locator.getByText`              |                |      |
| `Locator.getByTitle`             |                |      |
| `Locator.highlight`              |                |      |
| `Locator.hover`                  |                |      |
| `Locator.innerHTML`              |                |      |
| `Locator.innerText`              |                |      |
| `Locator.inputValue`             |                |      |
| `Locator.isChecked`              |                |      |
| `Locator.isDisabled`             |                |      |
| `Locator.isEditable`             |                |      |
| `Locator.isEnabled`              |                |      |
| `Locator.isHidden`               |                |      |
| `Locator.isVisible`              |                |      |
| `Locator.last`                   |                |      |
| `Locator.locator`                |                |      |
| `Locator.nth`                    |                |      |
| `Locator.or`                     |                |      |
| `Locator.page`                   |                |      |
| `Locator.press`                  |                |      |
| `Locator.pressSequentially`      |                |      |
| `Locator.screenshot`             |                |      |
| `Locator.scrollIntoViewIfNeeded` |                |      |
| `Locator.selectOption`           |                |      |
| `Locator.selectText`             |                |      |
| `Locator.setChecked`             |                |      |
| `Locator.setInputFiles`          |                |      |
| `Locator.tap`                    |                |      |
| `Locator.textContent`            |                |      |
| `Locator.uncheck`                |                |      |
| `Locator.waitFor`                |                |      |
| `Locator.elementHandle`          |                |      |
| `Locator.elementHandles`         |                |      |
| `Locator.type`                   |                |      |

## Logger

| Playweight         | WebDriver Bidi | Note |
| ------------------ | -------------- | ---- |
| `Logger.isEnabled` |                |      |
| `Logger.log`       |                |      |

## Mouse

| Playweight       | WebDriver Bidi | Note |
| ---------------- | -------------- | ---- |
| `Mouse.click`    |                |      |
| `Mouse.dblclick` |                |      |
| `Mouse.down`     |                |      |
| `Mouse.move`     |                |      |
| `Mouse.up`       |                |      |
| `Mouse.wheel`    |                |      |

## Page

| Playweight                         | WebDriver Bidi | Note |
| ---------------------------------- | -------------- | ---- |
| `Page.addInitScript`               |                |      |
| `Page.addLocatorHandler`           |                |      |
| `Page.addScriptTag`                |                |      |
| `Page.addStyleTag`                 |                |      |
| `Page.bringToFront`                |                |      |
| `Page.close`                       |                |      |
| `Page.consoleMessages`             |                |      |
| `Page.content`                     |                |      |
| `Page.context`                     |                |      |
| `Page.dragAndDrop`                 |                |      |
| `Page.emulateMedia`                |                |      |
| `Page.evaluate`                    |                |      |
| `Page.evaluateHandle`              |                |      |
| `Page.exposeBinding`               |                |      |
| `Page.exposeFunction`              |                |      |
| `Page.frame`                       |                |      |
| `Page.frameLocator`                |                |      |
| `Page.frames`                      |                |      |
| `Page.getByAltText`                |                |      |
| `Page.getByLabel`                  |                |      |
| `Page.getByPlaceholder`            |                |      |
| `Page.getByRole`                   |                |      |
| `Page.getByTestId`                 |                |      |
| `Page.getByText`                   |                |      |
| `Page.getByTitle`                  |                |      |
| `Page.goBack`                      |                |      |
| `Page.goForward`                   |                |      |
| `Page.goto`                        |                |      |
| `Page.isClosed`                    |                |      |
| `Page.locator`                     |                |      |
| `Page.mainFrame`                   |                |      |
| `Page.opener`                      |                |      |
| `Page.pageErrors`                  |                |      |
| `Page.pause`                       |                |      |
| `Page.pdf`                         |                |      |
| `Page.reload`                      |                |      |
| `Page.removeAllListeners`          |                |      |
| `Page.removeLocatorHandler`        |                |      |
| `Page.requestGC`                   |                |      |
| `Page.requests`                    |                |      |
| `Page.route`                       |                |      |
| `Page.routeFromHAR`                |                |      |
| `Page.routeWebSocket`              |                |      |
| `Page.screenshot`                  |                |      |
| `Page.setContent`                  |                |      |
| `Page.setDefaultNavigationTimeout` |                |      |
| `Page.setDefaultTimeout`           |                |      |
| `Page.setExtraHTTPHeaders`         |                |      |
| `Page.setViewportSize`             |                |      |
| `Page.title`                       |                |      |
| `Page.unroute`                     |                |      |
| `Page.unrouteAll`                  |                |      |
| `Page.url`                         |                |      |
| `Page.video`                       |                |      |
| `Page.viewportSize`                |                |      |
| `Page.waitForEvent`                |                |      |
| `Page.waitForFunction`             |                |      |
| `Page.waitForLoadState`            |                |      |
| `Page.waitForRequest`              |                |      |
| `Page.waitForResponse`             |                |      |
| `Page.waitForURL`                  |                |      |
| `Page.workers`                     |                |      |
| `Page.Properties`                  |                |      |
| `Page.clock`                       |                |      |
| `Page.coverage`                    |                |      |
| `Page.keyboard`                    |                |      |
| `Page.mouse`                       |                |      |
| `Page.request`                     |                |      |
| `Page.touchscreen`                 |                |      |
| `Page.on('close')`                 |                |      |
| `Page.on('console')`               |                |      |
| `Page.on('crash')`                 |                |      |
| `Page.on('dialog')`                |                |      |
| `Page.on('domcontentloaded')`      |                |      |
| `Page.on('download')`              |                |      |
| `Page.on('filechooser')`           |                |      |
| `Page.on('frameattached')`         |                |      |
| `Page.on('framedetached')`         |                |      |
| `Page.on('framenavigated')`        |                |      |
| `Page.on('load')`                  |                |      |
| `Page.on('pageerror')`             |                |      |
| `Page.on('popup')`                 |                |      |
| `Page.on('request')`               |                |      |
| `Page.on('requestfailed')`         |                |      |
| `Page.on('requestfinished')`       |                |      |
| `Page.on('response')`              |                |      |
| `Page.on('websocket')`             |                |      |
| `Page.on('worker')`                |                |      |
| `Page.$`                           |                |      |
| `Page.$$`                          |                |      |
| `Page.$eval`                       |                |      |
| `Page.$$eval`                      |                |      |
| `Page.accessibility`               |                |      |
| `Page.check`                       |                |      |
| `Page.click`                       |                |      |
| `Page.dblclick`                    |                |      |
| `Page.dispatchEvent`               |                |      |
| `Page.fill`                        |                |      |
| `Page.focus`                       |                |      |
| `Page.getAttribute`                |                |      |
| `Page.hover`                       |                |      |
| `Page.innerHTML`                   |                |      |
| `Page.innerText`                   |                |      |
| `Page.inputValue`                  |                |      |
| `Page.isChecked`                   |                |      |
| `Page.isDisabled`                  |                |      |
| `Page.isEditable`                  |                |      |
| `Page.isEnabled`                   |                |      |
| `Page.isHidden`                    |                |      |
| `Page.isVisible`                   |                |      |
| `Page.press`                       |                |      |
| `Page.selectOption`                |                |      |
| `Page.setChecked`                  |                |      |
| `Page.setInputFiles`               |                |      |
| `Page.tap`                         |                |      |
| `Page.textContent`                 |                |      |
| `Page.type`                        |                |      |
| `Page.uncheck`                     |                |      |
| `Page.waitForNavigation`           |                |      |
| `Page.waitForSelector`             |                |      |
| `Page.waitForTimeout`              |                |      |

## Request

| Playweight                    | WebDriver Bidi | Note |
| ----------------------------- | -------------- | ---- |
| `Request.allHeaders`          |                |      |
| `Request.failure`             |                |      |
| `Request.frame`               |                |      |
| `Request.headerValue`         |                |      |
| `Request.headers`             |                |      |
| `Request.headersArray`        |                |      |
| `Request.isNavigationRequest` |                |      |
| `Request.method`              |                |      |
| `Request.postData`            |                |      |
| `Request.postDataBuffer`      |                |      |
| `Request.postDataJSON`        |                |      |
| `Request.redirectedFrom`      |                |      |
| `Request.redirectedTo`        |                |      |
| `Request.resourceType`        |                |      |
| `Request.response`            |                |      |
| `Request.serviceWorker`       |                |      |
| `Request.sizes`               |                |      |
| `Request.timing`              |                |      |
| `Request.url`                 |                |      |

## Response

| Playweight                     | WebDriver Bidi | Note |
| ------------------------------ | -------------- | ---- |
| `Response.allHeaders`          |                |      |
| `Response.failure`             |                |      |
| `Response.frame`               |                |      |
| `Response.headerValue`         |                |      |
| `Response.headers`             |                |      |
| `Response.headersArray`        |                |      |
| `Response.isNavigationRequest` |                |      |
| `Response.method`              |                |      |
| `Response.postData`            |                |      |
| `Response.postDataBuffer`      |                |      |
| `Response.postDataJSON`        |                |      |
| `Response.redirectedFrom`      |                |      |
| `Response.redirectedTo`        |                |      |
| `Response.resourceType`        |                |      |
| `Response.response`            |                |      |
| `Response.serviceWorker`       |                |      |
| `Response.sizes`               |                |      |
| `Response.timing`              |                |      |
| `Response.url`                 |                |      |

## Route

| Playweight       | WebDriver Bidi | Note |
| ---------------- | -------------- | ---- |
| `Route.abort`    |                |      |
| `Route.continue` |                |      |
| `Route.fallback` |                |      |
| `Route.fetch`    |                |      |
| `Route.fulfill`  |                |      |
| `Route.request`  |                |      |

## Selectors

| Playweight                     | WebDriver Bidi | Note |
| ------------------------------ | -------------- | ---- |
| `Selectors.register`           |                |      |
| `Selectors.setTestIdAttribute` |                |      |

## Touchscreen

| Playweight        | WebDriver Bidi | Note |
| ----------------- | -------------- | ---- |
| `Touchscreen.tap` |                |      |

## Tracing

| Playweight           | WebDriver Bidi | Note |
| -------------------- | -------------- | ---- |
| `Tracing.group`      |                |      |
| `Tracing.groupEnd`   |                |      |
| `Tracing.start`      |                |      |
| `Tracing.startChunk` |                |      |
| `Tracing.stop`       |                |      |
| `Tracing.stopChunk`  |                |      |

## Video

| Playweight     | WebDriver Bidi | Note |
| -------------- | -------------- | ---- |
| `Video.delete` |                |      |
| `Video.path`   |                |      |
| `Video.saveAs` |                |      |

## WebError

| Playweight       | WebDriver Bidi | Note |
| ---------------- | -------------- | ---- |
| `WebError.error` |                |      |
| `WebError.page`  |                |      |

## WebSocket

| Playweight                      | WebDriver Bidi | Note |
| ------------------------------- | -------------- | ---- |
| `WebSocket.isClosed`            |                |      |
| `WebSocket.url`                 |                |      |
| `WebSocket.waitForEvent`        |                |      |
| `WebSocket.on('close')`         |                |      |
| `WebSocket.on('framereceived')` |                |      |
| `WebSocket.on('framesent')`     |                |      |
| `WebSocket.on('socketerror')`   |                |      |

## WebSocketRoute

| Playweight                       | WebDriver Bidi | Note |
| -------------------------------- | -------------- | ---- |
| `WebSocketRoute.close`           |                |      |
| `WebSocketRoute.connectToServer` |                |      |
| `WebSocketRoute.onClose`         |                |      |
| `WebSocketRoute.onMessage`       |                |      |
| `WebSocketRoute.send`            |                |      |
| `WebSocketRoute.url`             |                |      |

## Worker

| Playweight              | WebDriver Bidi | Note |
| ----------------------- | -------------- | ---- |
| `Worker.evaluate`       |                |      |
| `Worker.evaluateHandle` |                |      |
| `Worker.url`            |                |      |
| `Worker.on('close')`    |                |      |
