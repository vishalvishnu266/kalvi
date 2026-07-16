import axios from 'axios';

const API_BASE_URL = 'http://localhost:3000/api/tenant1'; // Hardcoded tenant for demo

const client = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});

export const StudentApi = {
    list: async (filters = {}) => {
        const response = await client.get('/students', { params: filters });
        return response.data;
    },
    get: async (id) => {
        const response = await client.get(`/students/${id}`);
        return response.data;
    },
    create: async (data) => {
        try {
            const response = await client.post('/students', data);
            return { data: response.data, error: null };
        } catch (err) {
            return { 
                data: null, 
                error: err.response?.data?.message || 'Failed to create student' 
            };
        }
    },
    update: async (id, data) => {
        try {
            const response = await client.put(`/students/${id}`, data);
            return { data: response.data, error: null };
        } catch (err) {
            return { 
                data: null, 
                error: err.response?.data?.message || 'Failed to update student' 
            };
        }
    },
    delete: async (id) => {
        await client.delete(`/students/${id}`);
    }
};
