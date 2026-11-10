import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'

// One route per stage of shopping.
export const routes: RouteRecordRaw[] = [
  { path: '/', redirect: '/pick' },
  { path: '/pick', name: 'pick', component: () => import('@/views/PickView.vue') },
  { path: '/shop', name: 'shop', component: () => import('@/views/ShopView.vue') },
  { path: '/fill-in', name: 'fill-in', component: () => import('@/views/FillInView.vue') },
]

// Hash history: GitHub Pages has no server-side rewrites for deep links.
const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
