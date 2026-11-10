// All persistence goes through this module. The list starts on this phone (localStorage),
// so the app works without an account; signed in, it lives in the user's own Firestore
// collection. Tests swap in unstorage's memory driver with `useDriver`.
// Facts about this browser alone (its device id) stay in localStorage and never sync.
import { createStorage, type Driver, type Storage } from 'unstorage'
import localStorageDriver from 'unstorage/drivers/localstorage'

import { getFirebase } from '@/firebase'
import firestoreDriver from './firestore'

const PREFIX = 'shopping-list'

const deviceStorage: Storage = createStorage({
  driver: localStorageDriver({ base: `${PREFIX}:device` }),
})
let storage: Storage = createStorage({ driver: localStorageDriver({ base: `${PREFIX}:list` }) })

export function getStorage(): Storage {
  return storage
}

export function getDeviceStorage(): Storage {
  return deviceStorage
}

/** Swap the storage backend, e.g. the memory driver in tests. */
export function useDriver(driver?: Driver): void {
  storage = createStorage(driver ? { driver } : {})
}

/** Keep the list on this phone only. */
export function useDeviceStorage(): void {
  useDriver(localStorageDriver({ base: `${PREFIX}:list` }))
}

/** Point storage at one signed-in user's own data in the cloud. */
export function useAccountStorage(uid: string): void {
  useDriver(firestoreDriver({ db: getFirebase().db, uid }))
}
