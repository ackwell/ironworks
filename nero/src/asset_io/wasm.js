export function pick_folder() {
  let resolve
  const promise = new Promise((resolvePromise) => {
    resolve = resolvePromise
  })

  const input = document.createElement('input')
  input.type = 'file'
  input.webkitdirectory = true
  input.addEventListener('change', event => {
    resolve(event.target.files)
  })

  input.click()

  return promise
}
