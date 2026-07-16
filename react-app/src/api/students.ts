import axios from 'axios';
import { Student, StudentFilters, DashboardStats } from '../types';

const API_BASE_URL = 'http://localhost:3000/api/tenant1';

const client = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});

export const StudentApi = {
    list: async (filters: StudentFilters = {}): Promise<Student[]> => {
        const response = await client.get<Student[]>('/students', { params: filters });
        return response.data;
    },
    get: async (id: string): Promise<Student> => {
        const response = await client.get<Student>(`/students/${id}`);
        return response.data;
    },
    create: async (data: Partial<Student>): Promise<Student> => {
        const response = await client.post<Student>('/students', data);
        return response.data;
    },
    update: async (id: string, data: Partial<Student>): Promise<Student> => {
        const response = await client.put<Student>(`/students/${id}`, data);
        return response.data;
    },
    delete: async (id: string): Promise<void> => {
        await client.delete(`/students/${id}`);
    },
    getDashboardStats: async (): Promise<DashboardStats> => {
        // We need to implement this in the Rust API, but for now we can mock or use a query
        // Actually, let's assume we'll add /api/{tenant}/dashboard-stats
        const response = await client.get<DashboardStats>('/dashboard-stats');
        return response.data;
    }
};
