<template>
  <div class="ui-datepicker">
    <span v-if="label" class="label">{{ label }}</span>

    <div class="trigger-input-group" :data-invalid="isInvalid">
      <input
          class="date-input"
          type="text"
          :value="modelValue"
          :placeholder="placeholder"
          @input="onManualInput"
          @blur="onInputBlur"
      />
      <button class="picker-btn" type="button" title="Open calendar" @click="openPop">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="16" y1="2" x2="16" y2="6"></line>
          <line x1="8" y1="2" x2="8" y2="6"></line>
          <line x1="3" y1="10" x2="21" y2="10"></line>
        </svg>
      </button>
    </div>

    <dialog
        ref="dialogRef"
        class="pop"
        @click="onDialogClick"
        @cancel.prevent="close"
    >
      <div class="drawer-head">
        <span class="title">{{ label || 'Select date' }}</span>
        <button class="drawer-close" title="Close" @click="close">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <div class="drawer-body">
        <div class="head">
          <button
              class="nav-btn"
              :title="mode === 'year' ? 'Previous 12 years' : 'Previous month'"
              @click="onNav(-1)"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="15 18 9 12 15 6"></polyline>
            </svg>
          </button>

          <span v-if="mode === 'year'" class="month" style="cursor: default">
            {{ yearPage }} – {{ yearPage + 11 }}
          </span>
          <button v-else class="month" title="Pick a year" @click="openYearPicker">
            {{ monthName }}
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
          </button>

          <button
              class="nav-btn"
              :title="mode === 'year' ? 'Next 12 years' : 'Next month'"
              @click="onNav(1)"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </button>
        </div>

        <!-- Year Grid Mode -->
        <div v-if="mode === 'year'" class="years">
          <button
              v-for="y in yearList"
              :key="y"
              :class="{ sel: y === viewYear }"
              @click="pickYear(y)"
          >
            {{ y }}
          </button>
        </div>

        <!-- Day Grid Mode -->
        <template v-else>
          <div class="dow">
            <div>M</div><div>T</div><div>W</div><div>T</div><div>F</div><div>S</div><div>S</div>
          </div>
          <div class="grid">
            <button
                v-for="(cell, index) in cells"
                :key="index"
                :class="getCellClasses(cell)"
                @click="pick(fmtISO(cell.date))"
            >
              {{ cell.d }}
            </button>
          </div>
        </template>

        <div class="foot">
          <button @click="today">Today</button>
          <button @click="clear">Clear</button>
        </div>
      </div>
    </dialog>
  </div>
</template>

<script>
export default {
  name: 'UiDatepicker',
  props: {
    label: { type: String, default: '' },
    modelValue: { type: String, default: '' },
    placeholder: { type: String, default: 'YYYY-MM-DD' },
    open: { type: Boolean, default: false }
  },
  emits: ['update:modelValue', 'update:open', 'ui-change'],
  data() {
    const now = new Date();
    return {
      isOpen: this.open,
      viewYear: now.getFullYear(),
      viewMonth: now.getMonth(),
      mode: 'day',
      yearPage: now.getFullYear() - (now.getFullYear() % 12),
      isInvalid: false
    };
  },
  computed: {
    parsedValue() {
      return this.parse(this.modelValue);
    },
    monthName() {
      return new Date(this.viewYear, this.viewMonth, 1).toLocaleString(undefined, {
        month: 'long',
        year: 'numeric'
      });
    },
    yearList() {
      return Array.from({ length: 12 }, (_, i) => this.yearPage + i);
    },
    cells() {
      const y = this.viewYear;
      const m = this.viewMonth;
      const first = new Date(y, m, 1);
      const startDow = (first.getDay() + 6) % 7;
      const daysInMonth = new Date(y, m + 1, 0).getDate();
      const prevDays = new Date(y, m, 0).getDate();
      const cells = [];

      for (let i = startDow - 1; i >= 0; i--) {
        cells.push({ d: prevDays - i, muted: true, date: new Date(y, m - 1, prevDays - i) });
      }
      for (let d = 1; d <= daysInMonth; d++) {
        cells.push({ d, muted: false, date: new Date(y, m, d) });
      }
      while (cells.length < 42) {
        const idx = cells.length - startDow - daysInMonth + 1;
        cells.push({ d: idx, muted: true, date: new Date(y, m + 1, idx) });
      }
      return cells;
    }
  },
  watch: {
    open(newVal) {
      this.isOpen = newVal;
    },
    isOpen(newVal) {
      this.$emit('update:open', newVal);
      this.syncDialogState(newVal);
    },
    modelValue: {
      immediate: true,
      handler(newVal) {
        const v = this.parse(newVal);
        if (v) {
          this.viewYear = v.getFullYear();
          this.viewMonth = v.getMonth();
          this.isInvalid = false;
        }
      }
    }
  },
  mounted() {
    if (this.isOpen) {
      this.syncDialogState(true);
    }
  },
  methods: {
    syncDialogState(openState) {
      const dialog = this.$refs.dialogRef;
      if (!dialog) return;

      if (openState) {
        if (!dialog.open) dialog.showModal();
      } else {
        if (dialog.open) dialog.close();
      }
    },
    openPop() {
      this.isOpen = true;
    },
    close() {
      this.isOpen = false;
    },
    onDialogClick(e) {
      const rect = e.currentTarget.getBoundingClientRect();
      const isInDialog =
          rect.top <= e.clientY &&
          e.clientY <= rect.top + rect.height &&
          rect.left <= e.clientX &&
          e.clientX <= rect.left + rect.width;

      if (!isInDialog) {
        this.close();
      }
    },
    onNav(delta) {
      if (this.mode === 'year') {
        this.yearPage += delta * 12;
        return;
      }
      this.viewMonth += delta;
      if (this.viewMonth < 0) {
        this.viewMonth = 11;
        this.viewYear--;
      } else if (this.viewMonth > 11) {
        this.viewMonth = 0;
        this.viewYear++;
      }
    },
    openYearPicker() {
      this.mode = 'year';
      this.yearPage = this.viewYear - (this.viewYear % 12);
    },
    pickYear(y) {
      this.viewYear = y;
      this.mode = 'day';
    },
    emitChange(val) {
      this.$emit('update:modelValue', val);
      this.$emit('ui-change', { value: val });
    },
    pick(iso) {
      this.isInvalid = false;
      this.emitChange(iso);
      this.close();
    },
    today() {
      const val = this.fmtISO(this.startOfDay(new Date()));
      this.isInvalid = false;
      this.emitChange(val);
      this.close();
    },
    clear() {
      this.isInvalid = false;
      this.emitChange('');
      this.close();
    },
    onManualInput(e) {
      const inputVal = e.target.value.trim();
      if (!inputVal) {
        this.isInvalid = false;
        if (this.modelValue !== '') {
          this.emitChange('');
        }
        return;
      }

      const isValidFormat = /^\d{4}-\d{2}-\d{2}$/.test(inputVal);
      const parsed = isValidFormat ? this.parse(inputVal) : null;

      if (parsed) {
        this.isInvalid = false;
        this.viewYear = parsed.getFullYear();
        this.viewMonth = parsed.getMonth();
        this.emitChange(inputVal);
      } else {
        this.isInvalid = true;
      }
    },
    onInputBlur(e) {
      const inputVal = e.target.value.trim();
      if (!inputVal) {
        this.isInvalid = false;
        return;
      }
      this.isInvalid = !this.parse(inputVal);
    },
    getCellClasses(cell) {
      const today = this.startOfDay(new Date());
      return {
        muted: cell.muted,
        today: this.sameDay(cell.date, today),
        sel: this.parsedValue && this.sameDay(cell.date, this.parsedValue)
      };
    },
    parse(s) {
      if (!s) return null;
      const [y, m, d] = s.split('-').map(Number);
      if (!y || !m || !d) return null;
      const dt = new Date(y, m - 1, d);
      return dt.getFullYear() === y && dt.getMonth() === m - 1 && dt.getDate() === d ? dt : null;
    },
    fmtISO(d) {
      const p = n => String(n).padStart(2, '0');
      return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}`;
    },
    sameDay(a, b) {
      return (
          a &&
          b &&
          a.getFullYear() === b.getFullYear() &&
          a.getMonth() === b.getMonth() &&
          a.getDate() === b.getDate()
      );
    },
    startOfDay(d) {
      return new Date(d.getFullYear(), d.getMonth(), d.getDate());
    }
  }
};
</script>

<style scoped>
.ui-datepicker {
  display: inline-block;
  position: relative;
  box-sizing: border-box;
  font-family: system-ui, -apple-system, sans-serif;
}

.label {
  display: block;
  font-size: var(--fs-xs, 12px);
  color: var(--color-text-muted, #666);
  margin-bottom: 6px;
  font-weight: var(--fw-medium, 500);
}

.trigger-input-group {
  display: inline-flex;
  align-items: center;
  background: var(--color-surface, #fff);
  border: 1px solid var(--color-border-strong, #ccc);
  border-radius: var(--radius-md, 6px);
  height: 40px;
  padding: 0 4px 0 12px;
  min-width: 180px;
  box-sizing: border-box;
}

.trigger-input-group[data-invalid="true"] {
  border-color: #dc2626 !important;
}

.trigger-input-group[data-invalid="true"] .date-input {
  color: #dc2626;
}

.date-input {
  flex: 1;
  border: none;
  background: transparent;
  font: inherit;
  font-size: var(--fs-sm, 14px);
  color: var(--color-text, #111);
  outline: none;
  width: 100%;
  min-width: 0;
}

.picker-btn {
  appearance: none;
  border: 0;
  background: transparent;
  cursor: pointer;
  padding: 6px;
  color: var(--color-text-muted, #666);
  border-radius: var(--radius-sm, 4px);
  display: grid;
  place-items: center;
}

/* Dialog Styles */
.pop {
  position: fixed;
  inset: auto 0 0 0;
  width: 100vw;
  max-width: 100vw;
  max-height: 85dvh;
  margin: 0;
  padding: 0;
  border: none;
  border-top-left-radius: 16px;
  border-top-right-radius: 16px;
  box-shadow: 0 -4px 24px rgba(0, 0, 0, 0.15);
  background: var(--color-surface, #fff);
  flex-direction: column;
  overflow: hidden;
}

.pop[open] {
  display: flex;
}

.pop::backdrop {
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(2px);
}

@media (min-width: 640px) {
  .pop {
    inset: 50% auto auto 50%;
    transform: translate(-50%, -50%);
    width: 360px;
    max-width: 90vw;
    border-radius: 12px;
    border: 1px solid var(--color-border, #eee);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.2);
  }
}

.drawer-head {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 16px;
  border-bottom: 1px solid var(--color-border, #eee);
}

.drawer-head .title {
  flex: 1;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text, #111);
}

.drawer-close {
  appearance: none;
  border: 0;
  background: transparent;
  cursor: pointer;
  color: #666;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  display: grid;
  place-items: center;
}

.drawer-body {
  padding: 12px 16px;
  overflow-y: auto;
  flex: 1;
}

.head {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-bottom: 8px;
}

.head .month {
  appearance: none;
  border: 0;
  background: transparent;
  cursor: pointer;
  font: inherit;
  font-weight: 600;
  font-size: 14px;
  color: var(--color-text, #111);
  padding: 4px 8px;
  border-radius: 6px;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  flex: 1;
  justify-content: center;
}

.nav-btn {
  appearance: none;
  border: 0;
  background: transparent;
  cursor: pointer;
  color: #666;
  width: 26px;
  height: 26px;
  border-radius: 6px;
  display: grid;
  place-items: center;
}

.years {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 4px;
  padding: 4px 0;
}

.years button {
  appearance: none;
  border: 0;
  background: transparent;
  font: inherit;
  font-size: 12px;
  color: var(--color-text, #111);
  height: 36px;
  cursor: pointer;
  border-radius: 6px;
}

.years button.sel {
  background: var(--color-primary, #2563eb);
  color: #fff;
  font-weight: 600;
}

.dow,
.grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
}

.dow div {
  font-size: 10px;
  color: #888;
  text-align: center;
  padding: 4px 0;
  font-weight: 500;
}

.grid button {
  appearance: none;
  border: 0;
  background: transparent;
  font: inherit;
  font-size: 12px;
  color: var(--color-text, #111);
  height: 36px;
  cursor: pointer;
  border-radius: 6px;
  display: grid;
  place-items: center;
}

.grid button.muted {
  color: #aaa;
}

.grid button.today {
  box-shadow: inset 0 0 0 1px var(--color-primary, #2563eb);
  color: var(--color-primary, #2563eb);
  font-weight: 600;
}

.grid button.sel {
  background: var(--color-primary, #2563eb);
  color: #fff;
  font-weight: 600;
}

.foot {
  margin-top: 10px;
  display: flex;
  justify-content: space-between;
  gap: 8px;
}

.foot button {
  appearance: none;
  border: 0;
  background: transparent;
  cursor: pointer;
  color: var(--color-primary, #2563eb);
  font: inherit;
  font-size: 12px;
  font-weight: 500;
}
</style>