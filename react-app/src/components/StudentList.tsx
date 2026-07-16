import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { StudentApi } from '../api/students';
import { Plus, Search, Edit, Trash2 } from 'lucide-react';

const StudentList = () => {
    const navigate = useNavigate();
    const queryClient = useQueryClient();
    const [search, setSearch] = useState('');

    const { data: students = [], isLoading } = useQuery({
        queryKey: ['students', search],
        queryFn: () => StudentApi.list({ q: search })
    });

    const deleteMutation = useMutation({
        mutationFn: StudentApi.delete,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['students'] });
            queryClient.invalidateQueries({ queryKey: ['dashboard-stats'] });
        }
    });

    const handleDelete = async (id: string) => {
        if (window.confirm('Are you sure you want to delete this student?')) {
            deleteMutation.mutate(id);
        }
    };

    return (
        <div className="card shadow-sm border-0">
            <div className="card-header bg-white d-flex justify-content-between align-items-center py-3 border-0">
                <h5 className="mb-0 fw-bold text-dark">Students</h5>
                <button className="btn btn-primary d-flex align-items-center" onClick={() => navigate('/students/new')}>
                    <Plus size={18} className="me-1" /> Add Student
                </button>
            </div>
            <div className="card-body">
                <div className="input-group mb-4 shadow-sm rounded">
                    <span className="input-group-text bg-white border-end-0">
                        <Search size={18} className="text-muted" />
                    </span>
                    <input 
                        type="text" 
                        className="form-control border-start-0 ps-0" 
                        placeholder="Search by name or admission no..." 
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                    />
                </div>
                
                {isLoading ? (
                    <div className="text-center py-5">
                        <div className="spinner-border text-primary" role="status">
                            <span className="visually-hidden">Loading...</span>
                        </div>
                    </div>
                ) : (
                    <div className="table-responsive">
                        <table className="table table-hover align-middle">
                            <thead className="table-light border-0">
                                <tr>
                                    <th>Admission No</th>
                                    <th>Name</th>
                                    <th>Class</th>
                                    <th>Status</th>
                                    <th className="text-end">Actions</th>
                                </tr>
                            </thead>
                            <tbody className="border-top-0">
                                {students.map(student => (
                                    <tr key={student.id}>
                                        <td><code className="bg-light px-2 py-1 rounded">{student.admission_no}</code></td>
                                        <td className="fw-medium">{student.first_name} {student.last_name}</td>
                                        <td>{student.class_name}-{student.section}</td>
                                        <td>
                                            <span className={`badge rounded-pill ${
                                                student.status === 'Active' ? 'bg-success-subtle text-success' : 
                                                student.status === 'Pending' ? 'bg-warning-subtle text-warning' :
                                                'bg-secondary-subtle text-secondary'
                                            }`}>
                                                {student.status}
                                            </span>
                                        </td>
                                        <td className="text-end">
                                            <div className="d-flex justify-content-end gap-1">
                                                <button className="btn btn-light btn-sm p-2 rounded-circle" onClick={() => navigate(`/students/edit/${student.id}`)}>
                                                    <Edit size={16} className="text-primary" />
                                                </button>
                                                <button className="btn btn-light btn-sm p-2 rounded-circle" onClick={() => handleDelete(student.id)}>
                                                    <Trash2 size={16} className="text-danger" />
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                ))}
                                {students.length === 0 && (
                                    <tr>
                                        <td colSpan={5} className="text-center py-5 text-muted">
                                            No students found matching your search.
                                        </td>
                                    </tr>
                                )}
                            </tbody>
                        </table>
                    </div>
                )}
            </div>
        </div>
    );
};

export default StudentList;
