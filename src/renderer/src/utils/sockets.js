export const registerSockets = (sockets, proxy) => {
  sockets &&
    Object.keys(sockets).forEach((t) => {
      'subscribe' !== t &&
        'unsubscribe' !== t &&
        proxy.$socket.emitter.addListener(t, sockets[t], proxy)
    })
}

export const destroySockets = (sockets, proxy) => {
  sockets &&
    Object.keys(sockets).forEach((t) => {
      proxy.$socket.emitter.removeListener(t, proxy)
    })
}
