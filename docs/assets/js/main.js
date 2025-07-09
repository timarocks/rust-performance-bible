// Theme Toggle
const themeToggle = document.querySelector('.theme-toggle');
const prefersDarkScheme = window.matchMedia('(prefers-color-scheme: dark)');
const currentTheme = localStorage.getItem('theme') || 'system';
let isDarkMode = false;

// Apply saved theme preference
function applyTheme(theme) {
  if (theme === 'dark' || (theme === 'system' && prefersDarkScheme.matches)) {
    document.documentElement.setAttribute('data-theme', 'dark');
    isDarkMode = true;
  } else {
    document.documentElement.setAttribute('data-theme', 'light');
    isDarkMode = false;
  }
  updateThemeToggleIcon();
}

// Update theme toggle icon
function updateThemeToggleIcon() {
  if (!themeToggle) return;
  const icon = themeToggle.querySelector('i');
  if (!icon) return;
  
  if (isDarkMode) {
    icon.className = 'fas fa-sun';
    icon.setAttribute('title', 'Switch to light mode');
  } else {
    icon.className = 'fas fa-moon';
    icon.setAttribute('title', 'Switch to dark mode');
  }
}

// Initialize theme
applyTheme(currentTheme);

// Toggle theme
if (themeToggle) {
  themeToggle.addEventListener('click', () => {
    const newTheme = isDarkMode ? 'light' : 'dark';
    localStorage.setItem('theme', newTheme);
    applyTheme(newTheme);
  });
}

// Handle system theme changes
prefersDarkScheme.addListener((e) => {
  if (currentTheme === 'system') {
    applyTheme('system');
  }
});

// Copy code blocks
function initCodeBlocks() {
  document.querySelectorAll('.code-block').forEach((block) => {
    const copyBtn = document.createElement('button');
    copyBtn.className = 'copy-btn';
    copyBtn.innerHTML = '<i class="far fa-copy"></i>';
    copyBtn.setAttribute('title', 'Copy to clipboard');
    
    const code = block.querySelector('code');
    if (!code) return;
    
    const toolbar = document.createElement('div');
    toolbar.className = 'code-toolbar';
    toolbar.appendChild(copyBtn);
    
    block.insertBefore(toolbar, code);
    
    copyBtn.addEventListener('click', () => {
      navigator.clipboard.writeText(code.textContent).then(() => {
        const originalTitle = copyBtn.getAttribute('title');
        copyBtn.innerHTML = '<i class="fas fa-check"></i>';
        copyBtn.setAttribute('title', 'Copied!');
        
        setTimeout(() => {
          copyBtn.innerHTML = '<i class="far fa-copy"></i>';
          copyBtn.setAttribute('title', originalTitle);
        }, 2000);
      });
    });
  });
}

// Initialize components when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  initCodeBlocks();
  
  // Initialize mobile menu if it exists
  const mobileMenuButton = document.querySelector('.mobile-menu-button');
  const mobileMenu = document.querySelector('.mobile-menu');
  
  if (mobileMenuButton && mobileMenu) {
    mobileMenuButton.addEventListener('click', () => {
      mobileMenu.classList.toggle('hidden');
    });
  }
  
  // Add smooth scrolling for anchor links
  document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
      e.preventDefault();
      
      const targetId = this.getAttribute('href');
      if (targetId === '#') return;
      
      const targetElement = document.querySelector(targetId);
      if (targetElement) {
        targetElement.scrollIntoView({
          behavior: 'smooth',
          block: 'start'
        });
        
        // Update URL without jumping
        if (history.pushState) {
          history.pushState(null, null, targetId);
        } else {
          location.hash = targetId;
        }
      }
    });
  });
});

// Load and display benchmark data
async function loadBenchmarkData(chapterId) {
  try {
    const response = await fetch(`/benchmarks/data/${chapterId}.json`);
    if (!response.ok) throw new Error('Benchmark data not found');
    
    const data = await response.json();
    renderBenchmarkCharts(data);
  } catch (error) {
    console.error('Error loading benchmark data:', error);
    const container = document.getElementById('benchmark-container');
    if (container) {
      container.innerHTML = '<p>Unable to load benchmark data. Please try again later.</p>';
    }
  }
}

// Render benchmark charts using Chart.js
function renderBenchmarkCharts(data) {
  if (!window.Chart) {
    console.error('Chart.js is not loaded');
    return;
  }
  
  // Example chart rendering - customize based on your benchmark data structure
  const ctx = document.getElementById('benchmark-chart');
  if (!ctx) return;
  
  // Destroy existing chart if it exists
  if (ctx.chart) {
    ctx.chart.destroy();
  }
  
  // Create new chart
  ctx.chart = new Chart(ctx, {
    type: 'bar',
    data: {
      labels: data.labels || [],
      datasets: [
        {
          label: 'Naive Implementation',
          data: data.naive || [],
          backgroundColor: 'rgba(255, 99, 132, 0.7)',
          borderColor: 'rgba(255, 99, 132, 1)',
          borderWidth: 1
        },
        {
          label: 'Optimized Implementation',
          data: data.optimized || [],
          backgroundColor: 'rgba(54, 162, 235, 0.7)',
          borderColor: 'rgba(54, 162, 235, 1)',
          borderWidth: 1
        }
      ]
    },
    options: {
      responsive: true,
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Time (ms)'
          }
        },
        x: {
          title: {
            display: true,
            text: 'Input Size'
          }
        }
      },
      plugins: {
        title: {
          display: true,
          text: data.title || 'Performance Comparison',
          font: {
            size: 18
          }
        },
        tooltip: {
          mode: 'index',
          intersect: false
        },
        legend: {
          position: 'top',
        }
      }
    }
  });
}

// Initialize benchmark charts if on a benchmark page
if (document.body.classList.contains('benchmark-page')) {
  const chapterId = document.body.getAttribute('data-chapter-id');
  if (chapterId) {
    // Load Chart.js if not already loaded
    if (typeof Chart === 'undefined') {
      const script = document.createElement('script');
      script.src = 'https://cdn.jsdelivr.net/npm/chart.js';
      script.onload = () => loadBenchmarkData(chapterId);
      document.head.appendChild(script);
    } else {
      loadBenchmarkData(chapterId);
    }
  }
}
