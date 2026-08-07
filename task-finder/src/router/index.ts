import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
    { path: '/', redirect: '/home' },
    {
        path: '/home',
        name: 'home',
        component: () => import('../pages/HomePage.vue'),
        meta: { title: 'DailyGig', tab: 'home' },
    },
    {
        path: '/tasks',
        name: 'tasks',
        component: () => import('../pages/TasksPage.vue'),
        meta: { title: 'Nearby Tasks', tab: 'tasks' },
    },
    {
        path: '/rides',
        name: 'rides',
        component: () => import('../pages/RidesPage.vue'),
        meta: { title: 'Rides Near You', tab: 'rides' },
    },
    {
        path: '/settings',
        name: 'settings',
        component: () => import('../pages/SettingsPage.vue'),
        meta: { title: 'Settings', tab: 'settings' },
    },
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes,
});
