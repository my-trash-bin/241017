# test 241017

1. run `dev.sh`
2. open the page using browser
3. open developer tools, go to console tab
4. click anywhere in the page to log
5. enter below to clear data

```js
(async () => {
    await (await navigator.storage.getDirectory()).remove({ recursive: true });
})();
```
