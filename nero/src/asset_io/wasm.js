const fileListKey = Symbol('fileInputKey')

export function pick_folder() {
  // If the file list already exists, use that instance.
  // TODO: Work out how this interacts with re-selecting sqpack or incorrect folders or w/e.
  let files = window[fileListKey]
  if (files != null) {
    return Promise.resolve(files)
  }

  // Build & extract a promise.
  let resolve
  const promise = new Promise((resolvePromise) => {
    resolve = resolvePromise
  })

  // Set up a folder picker input.
  const input = document.createElement('input')
  input.type = 'file'
  input.webkitdirectory = true
  // TODO: Look into rejecting promise if action is cancelled, can handle result on rust side.
  input.addEventListener('change', ({target: {files}}) => {
    window[fileListKey] = files
    resolve(files)
  })

  input.click()

  return promise
}
