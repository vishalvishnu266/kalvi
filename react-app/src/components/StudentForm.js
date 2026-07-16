import React, { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { StudentApi } from '../api/students';

const StudentForm = () => {
    const { id } = useParams();
    const navigate = useNavigate();
    const isEdit = !!id;
    
    const [formData, setFormData] = useState({
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
    const [error, setError] = useState(null);
    const [loading, setLoading] = useState(isEdit);
    const [submitting, setSubmitting] = useState(false);

    useEffect(() => {
        if (isEdit) {
            const fetchStudent = async () => {
                try {
                    const data = await StudentApi.get(id);
                    setFormData(data);
                } catch (err) {
                    setError('Failed to load student data');
                }
                setLoading(false);
            };
            fetchStudent();
        }
    }, [id, isEdit]);

    const handleChange = (e) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setError(null);
        setSubmitting(true);

        const result = isEdit 
            ? await StudentApi.update(id, formData)
            : await StudentApi.create(formData);

        if (result.error) {
            setError(result.error);
        } else {
            navigate('/students');
        }
        setSubmitting(false);
    };

    if (loading) return <div className="text-center py-5">Loading student data...</div>;

    return (
        <div className="card shadow-sm">
            <div className="card-header bg-white py-3">
                <h5 className="mb-0 fw-bold">{isEdit ? 'Edit Student' : 'Add Student'}</h5>
            </div>
            <form onSubmit={handleSubmit}>
                <div className="card-body">
                    {error && <div className="alert alert-danger mb-4">{error}</div>}
                    
                    <div className="row g-3">
                        <div className="col-md-6">
                            <label className="form-label">First Name *</label>
                            <input type="text" name="first_name" className="form-control" required
                                value={formData.first_name} onChange={handleChange} />
                        </div>
                        <div className="col-md-6">
                            <label className="form-label">Last Name *</label>
                            <input type="text" name="last_name" className="form-control" required
                                value={formData.last_name} onChange={handleChange} />
                        </div>
                        <div className="col-md-6">
                            <label className="form-label">Admission No *</label>
                            <input type="text" name="admission_no" className="form-control" required
                                value={formData.admission_no} onChange={handleChange} />
                        </div>
                        <div className="col-md-3">
                            <label className="form-label">Class *</label>
                            <input type="text" name="class_name" className="form-control" required
                                value={formData.class_name} onChange={handleChange} />
                        </div>
                        <div className="col-md-3">
                            <label className="form-label">Section *</label>
                            <input type="text" name="section" className="form-control" required
                                value={formData.section} onChange={handleChange} />
                        </div>
                    </div>
                </div>
                <div className="card-footer bg-white border-top-0 d-flex justify-content-end gap-2 py-3">
                    <button type="button" className="btn btn-light" onClick={() => navigate('/students')} disabled={submitting}>Cancel</button>
                    <button type="submit" className="btn btn-primary" disabled={submitting}>
                        {submitting ? 'Saving...' : 'Save Student'}
                    </button>
                </div>
            </form>
        </div>
    );
};

export default StudentForm;
