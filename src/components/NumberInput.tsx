// src/components/NumberInput.tsx
import React from 'react';
import '../styles/numberinput.css';

interface NumberInputProps {
  label: string;
  value: number;
  onChange: (value: number) => void;
}

const NumberInput: React.FC<NumberInputProps> = ({
  label,
  value,
  onChange,
}) => {
  return (
    <div className="number-input-group">
      <label className="number-input-label">{label}</label>
      <input
        type="number"
        className="number-input-field"
        value={value}
        onChange={(e) => {
          const newValue = parseInt(e.target.value) || 0;
          onChange(newValue);
        }}
      />
    </div>
  );
};

export default NumberInput;