import React, { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { StudentApi } from '../api/students';
import { ArrowLeft, Save } from 'lucide-react';
import { NativeBridge } from '../utils/NativeBridge';
import { Student } from '../types';

const StudentForm = () => {
    const { id } = useParams<{ id: string }>();
    const navigate = useNavigate();
    const queryClient = useQueryClient();
    const isEdit = !!id;
    
    const [formData, setFormData] = useState<Partial<Student>>({
        first_name: '',
        last_name: '',
        admission_no: '',
        class_name: '1',
        section: 'A',
        roll_no: '',
        gender: 'Male',
        guardian_name: '',
        guardian_phone: '',
        status: 'Active'
    });
    const [error, setError] = useState<string | null>(null);

    const { data: student, isLoading } = useQuery({
        queryKey: ['student', id],
        queryFn: () => StudentApi.get(id!),
        enabled: isEdit,
    });

    useEffect(() => {
        if (student) {
            setFormData(student);
        }
    }, [student]);

    const saveMutation = useMutation({
        mutationFn: (data: Partial<Student>) => 
            isEdit ? StudentApi.update(id!, data) : StudentApi.create(data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['students'] });
            queryClient.invalidateQueries({ queryKey: ['dashboard-stats'] });
            NativeBridge.hapticSuccess();
            navigate('/students');
        },
        onError: (err: any) => {
            NativeBridge.hapticError();
            setError(err.response?.data?.message || 'Failed to save student data');
        }
    });

    const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        saveMutation.mutate(formData);
    };

    if (isLoading) return <div className="text-center py-5">Loading student data...</div>;

    return (
        <div className="card shadow-sm border-0">
            <div className="card-header bg-white py-3 d-flex align-items-center border-0">
                <button className="btn btn-link text-muted p-0 me-3" onClick={() => navigate('/students')}>
                    <ArrowLeft size={20} />
                </button>
                <h5 className="mb-0 fw-bold">{isEdit ? 'Edit Student' : 'Add New Student'}</h5>
            </div>
            <form onSubmit={handleSubmit}>
                <div className="card-body p-4">
                    {error && <div className="alert alert-danger mb-4 border-0 shadow-sm">{error}</div>}
                    
                    <div className="row g-4">
                        <div className="col-md-6">
                            <label className="form-label fw-medium small text-muted">FIRST NAME *</label>
                            <input type="text" name="first_name" className="form-control bg-light border-0" required
                                value={formData.first_name} onChange={handleChange} />
                        </div>
                        <div className="col-md-6">
                            <label className="form-label fw-medium small text-muted">LAST NAME *</label>
                            <input type="text" name="last_name" className="form-control bg-light border-0" required
                                value={formData.last_name} onChange={handleChange} />
                        </div>
                        <div className="col-md-6">
                            <label className="form-label fw-medium small text-muted">ADMISSION NO *</label>
                            <input type="text" name="admission_no" className="form-control bg-light border-0" required
                                value={formData.admission_no} onChange={handleChange} />
                        </div>
                        <div className="col-md-3">
                            <label className="form-label fw-medium small text-muted">CLASS *</label>
                            <input type="text" name="class_name" className="form-control bg-light border-0" required
                                value={formData.class_name} onChange={handleChange} />
                        </div>
                        <div className="col-md-3">
                            <label className="form-label fw-medium small text-muted">SECTION *</label>
                            <input type="text" name="section" className="form-control bg-light border-0" required
                                value={formData.section} onChange={handleChange} />
                        </div>
                        <div className="col-md-6">
                            <label className="form-label fw-medium small text-muted">STATUS</label>
                            <select name="status" className="form-select bg-light border-0" value={formData.status} onChange={handleChange}>
                                <option value="Active">Active</option>
                                <option value="Pending">Pending</option>
                                <option value="Inactive">Inactive</option>
                            </select>
                        </div>
                    </div>
                </div>
                <div className="card-footer bg-white border-0 d-flex justify-content-end gap-2 p-4">
                    <button type="button" className="btn btn-light px-4" onClick={() => navigate('/students')} disabled={saveMutation.isPending}>Cancel</button>
                    <button type="submit" className="btn btn-primary px-4 d-flex align-items-center" disabled={saveMutation.isPending}>
                        {saveMutation.isPending ? 'Saving...' : <><Save size={18} className="me-2" /> Save Student</>}
                    </button>
                </div>
            </form>
        </div>
    );
};

export default StudentForm;
