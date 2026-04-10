<template>
  <div class="min-h-screen bg-base-100 flex flex-col font-sans">
    <!-- Navbar -->
    <nav class="navbar bg-base-100/90 backdrop-blur-md shadow-sm fixed w-full z-50 top-0 border-b border-base-200">
      <div class="container mx-auto px-4 md:px-8 w-full flex justify-between items-center h-16">
        <div class="navbar-start flex items-center gap-3">
          <a class="link link-neutral text-2xl font-bold no-underline flex items-center gap-2" href="#">
            <span class="icon-[tabler--package] text-primary text-3xl"></span>
            WMS
          </a>
        </div>
        
        <div class="navbar-center hidden md:flex items-center">
          <ul class="menu md:menu-horizontal gap-2 text-base font-medium">
            <li><a href="#features" class="hover:bg-base-200 rounded-lg px-4 py-2">Features</a></li>
            <li><a href="#how-it-works" class="hover:bg-base-200 rounded-lg px-4 py-2">How It Works</a></li>
            <li><a href="#pricing" class="hover:bg-base-200 rounded-lg px-4 py-2">Pricing</a></li>
            <li><a href="#faq" class="hover:bg-base-200 rounded-lg px-4 py-2">FAQ</a></li>
          </ul>
        </div>
        
        <div class="navbar-end flex items-center justify-end gap-3">
          <!-- Mobile Menu Dropdown -->
          <div class="dropdown relative inline-flex md:hidden [--auto-close:inside] [--placement:bottom-end]">
            <button id="mobile-menu-btn" type="button" class="btn btn-ghost btn-circle dropdown-toggle" aria-haspopup="menu" aria-expanded="false" aria-label="Menu">
              <span class="icon-[tabler--menu-2] text-2xl"></span>
            </button>
            <ul class="dropdown-menu dropdown-open:opacity-100 hidden bg-base-100 shadow-xl rounded-box w-52 mt-2 p-2 border border-base-200" role="menu" aria-orientation="vertical" aria-labelledby="mobile-menu-btn">
              <li><a class="dropdown-item py-2" href="#features">Features</a></li>
              <li><a class="dropdown-item py-2" href="#how-it-works">How It Works</a></li>
              <li><a class="dropdown-item py-2" href="#pricing">Pricing</a></li>
              <li><a class="dropdown-item py-2" href="#faq">FAQ</a></li>
              <hr class="border-base-content/20 my-2" />
              <li><NuxtLink class="btn btn-primary w-full justify-center" to="/auth/login">Login</NuxtLink></li>
            </ul>
          </div>
          
          <NuxtLink to="/auth/login" class="btn btn-ghost text-base font-medium hidden md:inline-flex hover:bg-base-200">
            Login
          </NuxtLink>
          <NuxtLink to="#cta" class="btn btn-primary hidden md:inline-flex">
            Get Started
            <span class="icon-[tabler--arrow-right] size-4"></span>
          </NuxtLink>
        </div>
      </div>
    </nav>

    <!-- Hero Section -->
    <section class="pt-32 pb-20 md:pt-40 md:pb-28 bg-gradient-to-br from-base-100 via-base-100 to-base-200 relative overflow-hidden">
      <!-- Decorative background elements -->
      <div class="absolute top-20 right-10 w-64 h-64 bg-primary/10 rounded-full blur-3xl"></div>
      <div class="absolute bottom-10 left-10 w-80 h-80 bg-secondary/10 rounded-full blur-3xl"></div>
      
      <div class="container mx-auto px-4 relative z-10">
        <div class="flex flex-col lg:flex-row items-center gap-16 lg:gap-12">
          
          <div class="flex-1 text-center lg:text-left">
            <div class="badge badge-primary badge-soft px-3 py-1 mb-6 inline-flex gap-1.5 text-sm font-medium">
              <span class="icon-[tabler--sparkles] size-4"></span>
              AI-Powered Analytics
            </div>
            <h1 class="text-4xl sm:text-5xl lg:text-7xl font-extrabold mb-6 leading-tight tracking-tight text-base-content">
              Modern Warehouse <br class="hidden lg:block"/>
              <span class="text-transparent bg-clip-text bg-gradient-to-r from-primary to-secondary">Simplified</span>
            </h1>
            <p class="text-lg md:text-xl text-base-content/70 mb-10 max-w-2xl mx-auto lg:mx-0 leading-relaxed font-medium">
              Streamline operations, reduce costs, and scale your business with completely intelligent and real-time inventory management.
            </p>
            <div class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
              <a href="#cta" class="btn btn-primary btn-lg rounded-full px-8 shadow-lg shadow-primary/30">
                <span class="icon-[tabler--rocket] size-5"></span>
                Start Free Trial
              </a>
              <button class="btn btn-outline btn-lg rounded-full px-8 flex gap-2 items-center text-base-content/80 border-base-content/20 hover:border-base-content/40 hover:bg-base-200">
                <span class="icon-[tabler--player-play] size-5"></span>
                Watch Demo
              </button>
            </div>
            
            <div class="mt-12 flex flex-col sm:flex-row items-center gap-4 justify-center lg:justify-start">
              <div class="avatar-group -space-x-4">
                <div class="avatar">
                  <div class="w-12 h-12 rounded-full bg-primary text-primary-content font-bold flex items-center justify-center border-2 border-base-100 shadow-sm">JD</div>
                </div>
                <div class="avatar">
                  <div class="w-12 h-12 rounded-full bg-secondary text-secondary-content font-bold flex items-center justify-center border-2 border-base-100 shadow-sm">MK</div>
                </div>
                <div class="avatar">
                  <div class="w-12 h-12 rounded-full bg-accent text-accent-content font-bold flex items-center justify-center border-2 border-base-100 shadow-sm">SL</div>
                </div>
              </div>
              <div class="text-left w-full sm:w-auto">
                <div class="flex items-center gap-1 text-warning mb-1">
                  <span class="icon-[tabler--star-filled] size-4"></span>
                  <span class="icon-[tabler--star-filled] size-4"></span>
                  <span class="icon-[tabler--star-filled] size-4"></span>
                  <span class="icon-[tabler--star-filled] size-4"></span>
                  <span class="icon-[tabler--star-filled] size-4"></span>
                </div>
                <span class="text-sm font-medium text-base-content/70">Trusted by <strong class="text-base-content">500+</strong> warehouses</span>
              </div>
            </div>
          </div>
          
          <div class="flex-1 w-full max-w-xl mx-auto lg:max-w-none relative">
            <div class="relative bg-base-100 rounded-2xl border border-base-200 shadow-2xl shadow-base-content/5 overflow-hidden transform transition-transform hover:scale-[1.02] duration-300">
              <div class="bg-base-200/50 border-b border-base-200 p-3 flex items-center gap-2">
                <div class="flex gap-1.5 ml-2">
                  <div class="w-3 h-3 rounded-full bg-error"></div>
                  <div class="w-3 h-3 rounded-full bg-warning"></div>
                  <div class="w-3 h-3 rounded-full bg-success"></div>
                </div>
                <div class="text-xs text-base-content/50 font-medium ml-4 font-mono bg-base-100/50 px-3 py-1 rounded w-full max-w-xs text-center border border-base-200/50">app.warehouse.com</div>
              </div>
              
              <div class="p-6 bg-base-100">
                <div class="grid grid-cols-3 gap-4 mb-6">
                  <div class="bg-primary/5 rounded-xl p-4 border border-primary/10">
                    <div class="text-xs font-semibold text-primary/70 mb-1 uppercase tracking-wider">Total Items</div>
                    <div class="text-2xl font-bold text-base-content">12.4K</div>
                  </div>
                  <div class="bg-success/5 rounded-xl p-4 border border-success/10">
                    <div class="text-xs font-semibold text-success/70 mb-1 uppercase tracking-wider">Accuracy</div>
                    <div class="text-2xl font-bold text-base-content">98.5%</div>
                  </div>
                  <div class="bg-info/5 rounded-xl p-4 border border-info/10">
                    <div class="text-xs font-semibold text-info/70 mb-1 uppercase tracking-wider">Dispatched</div>
                    <div class="text-2xl font-bold text-base-content">342</div>
                  </div>
                </div>
                
                <div class="rounded-xl border border-base-200 p-4 relative h-40 overflow-hidden bg-base-50">
                  <div class="absolute bottom-0 left-0 w-full h-[85%] bg-gradient-to-t from-primary/10 to-transparent"></div>
                  <div class="h-full w-full flex items-end justify-between px-2 pb-2 gap-2 relative z-10">
                    <div class="w-full bg-primary rounded-t-sm h-[40%] hover:h-[45%] transition-all opacity-80"></div>
                    <div class="w-full bg-primary rounded-t-sm h-[60%] hover:h-[65%] transition-all opacity-90"></div>
                    <div class="w-full bg-primary rounded-t-sm h-[80%] hover:h-[85%] transition-all"></div>
                    <div class="w-full bg-primary rounded-t-sm h-[55%] hover:h-[60%] transition-all opacity-80"></div>
                    <div class="w-full bg-primary rounded-t-sm h-[70%] hover:h-[75%] transition-all opacity-90"></div>
                    <div class="w-full bg-secondary rounded-t-sm h-[95%] hover:h-[100%] transition-all"></div>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- Floating element -->
            <div class="absolute -right-6 -bottom-6 bg-base-100 p-4 rounded-xl shadow-xl border border-base-200 z-20 flex items-center gap-4 animate-[bounce_3s_ease-in-out_infinite]">
              <div class="w-12 h-12 bg-success/20 rounded-full flex items-center justify-center">
                <span class="icon-[tabler--check] text-success text-2xl"></span>
              </div>
              <div>
                <div class="text-sm font-bold">System Optimized</div>
                <div class="text-xs text-base-content/60 font-medium">Latency: 12ms</div>
              </div>
            </div>
          </div>
          
        </div>
      </div>
    </section>

    <!-- Logo Cloud -->
    <section class="py-10 bg-base-100 border-y border-base-200">
      <div class="container mx-auto px-4">
        <p class="text-center text-sm font-semibold text-base-content/50 mb-8 uppercase tracking-widest">Trusted by industry leaders worldwide</p>
        <div class="flex flex-wrap justify-center items-center gap-10 md:gap-20 opacity-60 grayscale hover:grayscale-0 transition-all duration-500">
          <div class="flex items-center gap-2 text-2xl font-bold hover:text-primary transition-colors cursor-pointer">
            <span class="icon-[tabler--brand-amazon] text-4xl"></span>
            <span class="hidden sm:inline">Amazon</span>
          </div>
          <div class="flex items-center gap-2 text-2xl font-bold hover:text-primary transition-colors cursor-pointer">
            <span class="icon-[tabler--brand-fedex] text-4xl"></span>
            <span class="hidden sm:inline">FedEx</span>
          </div>
          <div class="flex items-center gap-2 text-2xl font-bold hover:text-primary transition-colors cursor-pointer">
            <span class="icon-[tabler--brand-ups] text-4xl"></span>
            <span class="hidden sm:inline">UPS</span>
          </div>
          <div class="flex items-center gap-2 text-2xl font-bold hover:text-primary transition-colors cursor-pointer">
            <span class="icon-[tabler--brand-dhl] text-4xl"></span>
            <span class="hidden sm:inline">DHL</span>
          </div>
        </div>
      </div>
    </section>

    <!-- Features -->
    <section id="features" class="py-20 md:py-32 bg-base-200/50">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-3xl mx-auto mb-16">
          <div class="badge badge-primary badge-soft px-3 py-1 mb-4 font-medium">Features</div>
          <h2 class="text-3xl md:text-5xl font-extrabold mb-6 tracking-tight">Everything you need to succeed</h2>
          <p class="text-lg md:text-xl text-base-content/70 font-medium">Powerful features designed to streamline every single aspect of your modern warehouse management</p>
        </div>
        
        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
          <!-- Card 1 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-8">
              <div class="w-14 h-14 bg-primary/10 rounded-2xl flex items-center justify-center mb-6">
                <span class="icon-[tabler--package] text-3xl text-primary"></span>
              </div>
              <h3 class="card-title text-xl font-bold mb-3">Smart Inventory</h3>
              <p class="text-base-content/70 flex-grow font-medium leading-relaxed">Real-time tracking with barcode scanning, automated stock alerts, and machine-learning powered intelligent reordering.</p>
            </div>
          </div>
          
          <!-- Card 2 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-8">
              <div class="w-14 h-14 bg-success/10 rounded-2xl flex items-center justify-center mb-6">
                <span class="icon-[tabler--truck-delivery] text-3xl text-success"></span>
              </div>
              <h3 class="card-title text-xl font-bold mb-3">Movement Tracking</h3>
              <p class="text-base-content/70 flex-grow font-medium leading-relaxed">Monitor inbound and outbound logistics with complete visibility of every single shipment on transit.</p>
            </div>
          </div>
          
          <!-- Card 3 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-8">
              <div class="w-14 h-14 bg-info/10 rounded-2xl flex items-center justify-center mb-6">
                <span class="icon-[tabler--chart-bar] text-3xl text-info"></span>
              </div>
              <h3 class="card-title text-xl font-bold mb-3">Analytics Dashboard</h3>
              <p class="text-base-content/70 flex-grow font-medium leading-relaxed">Comprehensive reporting with customizable KPIs, predictive insights, and automated email summaries.</p>
            </div>
          </div>
          
          <!-- Card 4 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-8">
              <div class="w-14 h-14 bg-warning/10 rounded-2xl flex items-center justify-center mb-6">
                <span class="icon-[tabler--lock] text-3xl text-warning"></span>
              </div>
              <h3 class="card-title text-xl font-bold mb-3">Secure Access</h3>
              <p class="text-base-content/70 flex-grow font-medium leading-relaxed">Role-based permissions, comprehensive audit trails, and enterprise-grade security protocols standard.</p>
            </div>
          </div>
          
          <!-- Card 5 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-8">
              <div class="w-14 h-14 bg-secondary/10 rounded-2xl flex items-center justify-center mb-6">
                <span class="icon-[tabler--api] text-3xl text-secondary"></span>
              </div>
              <h3 class="card-title text-xl font-bold mb-3">API Integration</h3>
              <p class="text-base-content/70 flex-grow font-medium leading-relaxed">Connect with ERP, e-commerce, and shipping platforms seamlessly through our fully documented API.</p>
            </div>
          </div>
          
          <!-- Card 6 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-8">
              <div class="w-14 h-14 bg-accent/10 rounded-2xl flex items-center justify-center mb-6">
                <span class="icon-[tabler--device-mobile] text-3xl text-accent"></span>
              </div>
              <h3 class="card-title text-xl font-bold mb-3">Mobile Ready</h3>
              <p class="text-base-content/70 flex-grow font-medium leading-relaxed">Manage your warehouse on-the-go with our responsive, offline-first mobile web experience.</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Stats Matrix -->
    <section class="py-20 bg-base-content text-base-100 relative overflow-hidden">
      <!-- Decorative background -->
      <div class="absolute inset-0 opacity-10 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-base-100 via-transparent to-transparent bg-[length:20px_20px]"></div>
      
      <div class="container mx-auto px-4 relative z-10">
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-8 lg:gap-12 divide-x-0 divide-y lg:divide-y-0 lg:divide-x divide-base-100/20">
          <div class="text-center px-4 py-6 lg:py-0">
            <div class="text-5xl md:text-6xl font-black text-primary mb-2">500+</div>
            <div class="text-lg font-medium text-base-100/70">Warehouses Active</div>
          </div>
          <div class="text-center px-4 py-6 lg:py-0">
            <div class="text-5xl md:text-6xl font-black text-secondary mb-2">10M+</div>
            <div class="text-lg font-medium text-base-100/70">Items Tracked</div>
          </div>
          <div class="text-center px-4 py-6 lg:py-0">
            <div class="text-5xl md:text-6xl font-black text-success mb-2">99.9%</div>
            <div class="text-lg font-medium text-base-100/70">Guaranteed Uptime</div>
          </div>
          <div class="text-center px-4 py-6 lg:py-0">
            <div class="text-5xl md:text-6xl font-black text-accent mb-2">50+</div>
            <div class="text-lg font-medium text-base-100/70">Countries Served</div>
          </div>
        </div>
      </div>
    </section>

    <!-- FAQ Accordion -->
    <section id="faq" class="py-20 md:py-32 bg-base-100">
      <div class="container mx-auto px-4 max-w-4xl">
        <div class="text-center mb-16">
          <div class="badge badge-secondary badge-soft px-3 py-1 mb-4 font-medium">Questions</div>
          <h2 class="text-3xl md:text-5xl font-extrabold mb-6 tracking-tight">Frequently asked questions</h2>
          <p class="text-lg text-base-content/70 font-medium">Everything you need to know about our WMS platform and billing</p>
        </div>
        
        <!-- FlyonUI Responsive Accordion -->
        <div class="accordion divide-neutral/20 divide-y border-y border-neutral/20" id="landing-faq">
          <!-- Item 1 -->
          <div class="accordion-item active" id="faq-1">
            <button class="accordion-toggle inline-flex w-full items-center justify-between text-start text-lg font-semibold py-5 hover:text-primary transition-colors" aria-controls="faq-collapse-1" aria-expanded="true">
              How long does implementation and onboarding take?
              <span class="icon-[tabler--plus] accordion-item-active:hidden text-base-content size-5 block shrink-0"></span>
              <span class="icon-[tabler--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
            </button>
            <div id="faq-collapse-1" class="accordion-content w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-1" role="region">
              <div class="pb-6 pr-8">
                <p class="text-base-content/80 font-medium leading-relaxed">
                  Most implementations are completed within 2-4 weeks, depending on warehouse size and complexity. Our dedicated onboarding team will guide you through every step, configure your master data, and train your floor managers.
                </p>
              </div>
            </div>
          </div>
          
          <!-- Item 2 -->
          <div class="accordion-item" id="faq-2">
            <button class="accordion-toggle inline-flex w-full items-center justify-between text-start text-lg font-semibold py-5 hover:text-primary transition-colors" aria-controls="faq-collapse-2" aria-expanded="false">
              Is training provided for my staff?
              <span class="icon-[tabler--plus] accordion-item-active:hidden text-base-content size-5 block shrink-0"></span>
              <span class="icon-[tabler--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
            </button>
            <div id="faq-collapse-2" class="accordion-content hidden w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-2" role="region">
              <div class="pb-6 pr-8">
                <p class="text-base-content/80 font-medium leading-relaxed">
                  Absolutely! We provide comprehensive training including rich documentation, interactive video tutorials, and live Zoom sessions. Professional and Enterprise plans include dedicated 1-on-1 operational training sessions.
                </p>
              </div>
            </div>
          </div>
          
          <!-- Item 3 -->
          <div class="accordion-item" id="faq-3">
            <button class="accordion-toggle inline-flex w-full items-center justify-between text-start text-lg font-semibold py-5 hover:text-primary transition-colors" aria-controls="faq-collapse-3" aria-expanded="false">
              Can I import my existing inventory data?
              <span class="icon-[tabler--plus] accordion-item-active:hidden text-base-content size-5 block shrink-0"></span>
              <span class="icon-[tabler--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
            </button>
            <div id="faq-collapse-3" class="accordion-content hidden w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-3" role="region">
              <div class="pb-6 pr-8">
                <p class="text-base-content/80 font-medium leading-relaxed">
                  Yes, you can confidently import your data in bulk. We natively support imports from Excel, CSV datasets, and provide seamless, one-click synchronization loops with most major ERP architectures like SAP and Oracle.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- How It Works Section -->
    <section id="how-it-works" class="py-20 md:py-32 bg-base-100 relative overflow-hidden">
      <div class="container mx-auto px-4 relative z-10">
        <div class="text-center max-w-3xl mx-auto mb-16">
          <div class="badge badge-secondary badge-soft px-3 py-1 mb-4 font-medium">How It Works</div>
          <h2 class="text-3xl md:text-5xl font-extrabold mb-6 tracking-tight">Get started in 4 simple steps</h2>
          <p class="text-lg md:text-xl text-base-content/70 font-medium">From setup to full operation in less than 24 hours</p>
        </div>
        
        <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
          <!-- Step 1 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-6 text-center">
              <div class="w-16 h-16 bg-primary/10 rounded-2xl flex items-center justify-center mb-4 mx-auto">
                <span class="icon-[tabler--user-plus] text-3xl text-primary"></span>
              </div>
              <div class="badge badge-primary mb-3">Step 1</div>
              <h3 class="card-title text-lg font-bold mb-2 justify-center">Create Account</h3>
              <p class="text-base-content/70 text-sm">Sign up in seconds and configure your warehouse profile with basic information.</p>
            </div>
          </div>
          
          <!-- Step 2 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-6 text-center">
              <div class="w-16 h-16 bg-secondary/10 rounded-2xl flex items-center justify-center mb-4 mx-auto">
                <span class="icon-[tabler--building-warehouse] text-3xl text-secondary"></span>
              </div>
              <div class="badge badge-secondary mb-3">Step 2</div>
              <h3 class="card-title text-lg font-bold mb-2 justify-center">Setup Layout</h3>
              <p class="text-base-content/70 text-sm">Import your floor plan and define zones, aisles, and storage locations visually.</p>
            </div>
          </div>
          
          <!-- Step 3 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-6 text-center">
              <div class="w-16 h-16 bg-accent/10 rounded-2xl flex items-center justify-center mb-4 mx-auto">
                <span class="icon-[tabler--database-import] text-3xl text-accent"></span>
              </div>
              <div class="badge badge-accent mb-3">Step 3</div>
              <h3 class="card-title text-lg font-bold mb-2 justify-center">Import Data</h3>
              <p class="text-base-content/70 text-sm">Bulk upload your inventory via CSV or integrate with your existing ERP system.</p>
            </div>
          </div>
          
          <!-- Step 4 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
            <div class="card-body p-6 text-center">
              <div class="w-16 h-16 bg-success/10 rounded-2xl flex items-center justify-center mb-4 mx-auto">
                <span class="icon-[tabler--rocket] text-3xl text-success"></span>
              </div>
              <div class="badge badge-success mb-3">Step 4</div>
              <h3 class="card-title text-lg font-bold mb-2 justify-center">Go Live</h3>
              <p class="text-base-content/70 text-sm">Start managing operations with real-time tracking and analytics from day one.</p>
            </div>
          </div>
        </div>
        
        <!-- Arrow connectors for desktop -->
        <div class="hidden lg:flex justify-center items-center mt-8 gap-4">
          <div class="h-0.5 w-24 bg-gradient-to-r from-primary to-secondary rounded"></div>
          <span class="icon-[tabler--arrow-right] text-base-content/30"></span>
          <div class="h-0.5 w-24 bg-gradient-to-r from-secondary to-accent rounded"></div>
          <span class="icon-[tabler--arrow-right] text-base-content/30"></span>
          <div class="h-0.5 w-24 bg-gradient-to-r from-accent to-success rounded"></div>
        </div>
      </div>
    </section>

    <!-- Testimonials Section -->
    <section id="testimonials" class="py-20 md:py-32 bg-base-200/50 relative overflow-hidden">
      <!-- Decorative background -->
      <div class="absolute top-0 left-0 w-full h-32 bg-gradient-to-b from-base-100 to-transparent"></div>
      
      <div class="container mx-auto px-4 relative z-10">
        <div class="text-center max-w-3xl mx-auto mb-16">
          <div class="badge badge-primary badge-soft px-3 py-1 mb-4 font-medium">Testimonials</div>
          <h2 class="text-3xl md:text-5xl font-extrabold mb-6 tracking-tight">Loved by warehouse managers</h2>
          <p class="text-lg md:text-xl text-base-content/70 font-medium">See what our customers have to say about their experience</p>
        </div>
        
        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          <!-- Testimonial 1 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm">
            <div class="card-body p-6">
              <div class="flex items-center gap-1 text-warning mb-4">
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
              </div>
              <p class="text-base-content/80 mb-4 font-medium leading-relaxed">"WMS transformed our operations. We reduced picking errors by 85% and increased throughput by 40% in just 3 months."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-12 h-12 rounded-full bg-primary text-primary-content font-bold flex items-center justify-center">JD</div>
                </div>
                <div>
                  <p class="font-bold text-sm">John Davidson</p>
                  <p class="text-xs text-base-content/60">Operations Manager, TechLogistics</p>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Testimonial 2 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm">
            <div class="card-body p-6">
              <div class="flex items-center gap-1 text-warning mb-4">
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
              </div>
              <p class="text-base-content/80 mb-4 font-medium leading-relaxed">"The AI-powered inventory forecasting is a game-changer. We no longer run out of stock during peak seasons."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-12 h-12 rounded-full bg-secondary text-secondary-content font-bold flex items-center justify-center">SK</div>
                </div>
                <div>
                  <p class="font-bold text-sm">Sarah Kim</p>
                  <p class="text-xs text-base-content/60">Warehouse Director, FastShip Co</p>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Testimonial 3 -->
          <div class="card bg-base-100 border border-base-200 shadow-sm">
            <div class="card-body p-6">
              <div class="flex items-center gap-1 text-warning mb-4">
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="icon-[tabler--star-filled] size-4"></span>
              </div>
              <p class="text-base-content/80 mb-4 font-medium leading-relaxed">"Setup was incredibly fast. The mobile app lets our team work efficiently from anywhere in the warehouse."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-12 h-12 rounded-full bg-accent text-accent-content font-bold flex items-center justify-center">MR</div>
                </div>
                <div>
                  <p class="font-bold text-sm">Mike Rodriguez</p>
                  <p class="text-xs text-base-content/60">CEO, QuickStorage Solutions</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Pricing Section -->
    <section id="pricing" class="py-20 md:py-32 bg-base-100 relative overflow-hidden">
      <div class="container mx-auto px-4 relative z-10">
        <div class="text-center max-w-3xl mx-auto mb-16">
          <div class="badge badge-accent badge-soft px-3 py-1 mb-4 font-medium">Pricing</div>
          <h2 class="text-3xl md:text-5xl font-extrabold mb-6 tracking-tight">Simple, transparent pricing</h2>
          <p class="text-lg md:text-xl text-base-content/70 font-medium">Choose the plan that fits your warehouse size</p>
        </div>
        
        <div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto">
          <!-- Starter Plan -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-2 transition-all duration-300">
            <div class="card-body p-8">
              <div class="badge badge-ghost mb-4">Starter</div>
              <div class="flex items-baseline gap-1 mb-2">
                <span class="text-4xl font-black">$49</span>
                <span class="text-base-content/60">/month</span>
              </div>
              <p class="text-base-content/60 text-sm mb-6">Perfect for small warehouses</p>
              
              <ul class="space-y-3 mb-8">
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Up to 1,000 SKUs
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  3 User accounts
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Basic analytics
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Email support
                </li>
                <li class="flex items-center gap-2 text-sm text-base-content/40">
                  <span class="icon-[tabler--x]"></span>
                  API access
                </li>
              </ul>
              
              <button class="btn btn-outline w-full">Get Started</button>
            </div>
          </div>
          
          <!-- Professional Plan -->
          <div class="card bg-primary text-primary-content border-0 shadow-xl relative scale-105">
            <div class="absolute -top-4 left-1/2 -translate-x-1/2">
              <span class="badge badge-secondary badge-soft">Most Popular</span>
            </div>
            <div class="card-body p-8">
              <div class="badge badge-primary-content/20 mb-4 text-primary-content">Professional</div>
              <div class="flex items-baseline gap-1 mb-2">
                <span class="text-4xl font-black">$149</span>
                <span class="text-primary-content/70">/month</span>
              </div>
              <p class="text-primary-content/70 text-sm mb-6">For growing operations</p>
              
              <ul class="space-y-3 mb-8">
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-primary-content"></span>
                  Up to 10,000 SKUs
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-primary-content"></span>
                  15 User accounts
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-primary-content"></span>
                  Advanced analytics & AI
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-primary-content"></span>
                  Priority support
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-primary-content"></span>
                  API access
                </li>
              </ul>
              
              <button class="btn btn-primary-content w-full text-primary">Get Started</button>
            </div>
          </div>
          
          <!-- Enterprise Plan -->
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-xl hover:-translate-y-2 transition-all duration-300">
            <div class="card-body p-8">
              <div class="badge badge-ghost mb-4">Enterprise</div>
              <div class="flex items-baseline gap-1 mb-2">
                <span class="text-4xl font-black">$499</span>
                <span class="text-base-content/60">/month</span>
              </div>
              <p class="text-base-content/60 text-sm mb-6">For large-scale warehouses</p>
              
              <ul class="space-y-3 mb-8">
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Unlimited SKUs
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Unlimited users
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Custom integrations
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  24/7 dedicated support
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  On-premise deployment
                </li>
              </ul>
              
              <button class="btn btn-outline w-full">Contact Sales</button>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Integration Section -->
    <section id="integrations" class="py-20 md:py-32 bg-base-200/50 relative overflow-hidden">
      <div class="container mx-auto px-4 relative z-10">
        <div class="text-center max-w-3xl mx-auto mb-16">
          <div class="badge badge-secondary badge-soft px-3 py-1 mb-4 font-medium">Integrations</div>
          <h2 class="text-3xl md:text-5xl font-extrabold mb-6 tracking-tight">Connect with your favorite tools</h2>
          <p class="text-lg md:text-xl text-base-content/70 font-medium">Seamless integration with 100+ platforms and services</p>
        </div>
        
        <div class="grid grid-cols-3 md:grid-cols-6 gap-6 max-w-4xl mx-auto">
          <div class="card bg-base-100 border border-base-200 p-4 flex items-center justify-center hover:shadow-lg transition-all">
            <span class="icon-[tabler--brand-shopify] text-4xl text-success"></span>
          </div>
          <div class="card bg-base-100 border border-base-200 p-4 flex items-center justify-center hover:shadow-lg transition-all">
            <span class="icon-[tabler--brand-aws] text-4xl text-warning"></span>
          </div>
          <div class="card bg-base-100 border border-base-200 p-4 flex items-center justify-center hover:shadow-lg transition-all">
            <span class="icon-[tabler--brand-salesforce] text-4xl text-info"></span>
          </div>
          <div class="card bg-base-100 border border-base-200 p-4 flex items-center justify-center hover:shadow-lg transition-all">
            <span class="icon-[tabler--brand-slack] text-4xl text-secondary"></span>
          </div>
          <div class="card bg-base-100 border border-base-200 p-4 flex items-center justify-center hover:shadow-lg transition-all">
            <span class="icon-[tabler--brand-stripe] text-4xl text-primary"></span>
          </div>
          <div class="card bg-base-100 border border-base-200 p-4 flex items-center justify-center hover:shadow-lg transition-all">
            <span class="icon-[tabler--brand-zapier] text-4xl text-error"></span>
          </div>
        </div>
        
        <div class="text-center mt-8">
          <p class="text-base-content/60 text-sm">And many more... <a href="#" class="link link-primary">View all integrations</a></p>
        </div>
      </div>
    </section>

    <!-- Newsletter/Contact Section -->
    <section id="contact" class="py-20 md:py-32 bg-base-100 relative overflow-hidden">
      <div class="container mx-auto px-4 relative z-10">
        <div class="max-w-2xl mx-auto text-center">
          <div class="badge badge-primary badge-soft px-3 py-1 mb-4 font-medium">Stay Updated</div>
          <h2 class="text-3xl md:text-4xl font-extrabold mb-4 tracking-tight">Get the latest warehouse insights</h2>
          <p class="text-lg text-base-content/70 mb-8 font-medium">Subscribe to our newsletter for tips, industry news, and product updates.</p>
          
          <form class="flex flex-col sm:flex-row gap-3 max-w-md mx-auto" @submit.prevent>
            <div class="join w-full">
              <input type="email" placeholder="Enter your email" class="input input-bordered join-item flex-1" required />
              <button type="submit" class="btn btn-primary join-item">
                <span class="icon-[tabler--send] size-4"></span>
                Subscribe
              </button>
            </div>
          </form>
          
          <p class="text-xs text-base-content/50 mt-4">No spam, unsubscribe at any time. Read our <a href="#" class="link">Privacy Policy</a>.</p>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section id="cta" class="py-20 md:py-32 bg-primary relative overflow-hidden">
      <!-- Decorative BG -->
      <div class="absolute inset-0 opacity-20 hidden md:block">
        <svg class="h-full w-full" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="grid-pattern" width="40" height="40" patternUnits="userSpaceOnUse">
              <path d="M0 40L40 0H20L0 20M40 40V20L20 40" fill="none" stroke="currentColor" stroke-width="2"/>
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid-pattern)" class="text-primary-content"/>
        </svg>
      </div>

      <div class="container mx-auto px-4 text-center relative z-10">
        <h2 class="text-4xl md:text-5xl lg:text-6xl font-black text-primary-content mb-8 tracking-tight drop-shadow-sm">Ready to transform your <br class="hidden md:block"/>warehouse framework?</h2>
        <p class="text-xl text-primary-content/90 mb-10 max-w-2xl mx-auto font-medium">Join thousands of leading warehouses already using our ecosystem. Start your completely unrestricted free trial today.</p>
        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <a href="#" class="btn btn-lg bg-base-100 text-primary hover:bg-base-200 border-none rounded-full px-10 shadow-xl font-bold">
            <span class="icon-[tabler--rocket] mr-2 size-5"></span>
            Start 14-Day Free Trial
          </a>
          <a href="#" class="btn btn-lg btn-outline text-primary-content border-primary-content/30 hover:bg-primary-content/10 rounded-full px-10 font-bold">
            <span class="icon-[tabler--calendar] mr-2 size-5"></span>
            Schedule Demo
          </a>
        </div>
        <p class="mt-8 text-sm font-medium text-primary-content/70">No credit card required. Cancel anytime.</p>
      </div>
    </section>

    <!-- Footer -->
    <footer class="bg-base-200 text-base-content border-t border-base-200/50 pt-16 pb-8 mt-auto">
      <div class="container mx-auto px-4">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-12 gap-10 mb-12">
          <!-- Brand -->
          <div class="lg:col-span-4">
            <a class="flex items-center gap-2 text-2xl font-bold mb-6 text-base-content" href="#">
              <span class="icon-[tabler--package] text-primary text-3xl"></span>
              WMS
            </a>
            <p class="text-base-content/70 text-base font-medium mb-6 pr-4">
              Modern warehouse management architecture engineered for scalability, speed, and real-time reliability.
            </p>
            <div class="flex gap-3">
              <a href="#" class="btn btn-circle btn-soft btn-primary btn-sm">
                <span class="icon-[tabler--brand-twitter] size-4"></span>
              </a>
              <a href="#" class="btn btn-circle btn-soft btn-secondary btn-sm">
                <span class="icon-[tabler--brand-linkedin] size-4"></span>
              </a>
              <a href="#" class="btn btn-circle btn-soft btn-accent btn-sm">
                <span class="icon-[tabler--brand-github] size-4"></span>
              </a>
            </div>
          </div>
          
          <!-- Links -->
          <div class="lg:col-span-2 lg:col-start-7">
            <h4 class="font-bold mb-5 text-lg">Product</h4>
            <ul class="space-y-3 font-medium text-base-content/70">
              <li><a href="#features" class="hover:text-primary transition-colors">Features</a></li>
              <li><a href="#pricing" class="hover:text-primary transition-colors">Pricing</a></li>
              <li><a href="#" class="hover:text-primary transition-colors">API</a></li>
            </ul>
          </div>
          
          <div class="lg:col-span-2">
            <h4 class="font-bold mb-5 text-lg">Company</h4>
            <ul class="space-y-3 font-medium text-base-content/70">
              <li><a href="#" class="hover:text-primary transition-colors">About Us</a></li>
              <li><a href="#" class="hover:text-primary transition-colors">Blog</a></li>
              <li><a href="#" class="hover:text-primary transition-colors">Careers</a></li>
            </ul>
          </div>
          
          <div class="lg:col-span-2">
            <h4 class="font-bold mb-5 text-lg">Support</h4>
            <ul class="space-y-3 font-medium text-base-content/70">
              <li><a href="#" class="hover:text-primary transition-colors">Help Center</a></li>
              <li><a href="#" class="hover:text-primary transition-colors">System Status</a></li>
              <li><a href="#" class="hover:text-primary transition-colors">Security</a></li>
            </ul>
          </div>
        </div>
        
        <div class="border-t border-base-content/10 pt-8 flex flex-col md:flex-row justify-between items-center gap-4">
          <p class="text-sm font-medium text-base-content/60">&copy; 2026 Warehouse WMS. All rights reserved.</p>
          <div class="flex gap-6 text-sm font-medium text-base-content/60">
            <a href="#" class="hover:text-primary transition-colors">Privacy Policy</a>
            <a href="#" class="hover:text-primary transition-colors">Terms of Service</a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { onMounted } from 'vue'

definePageMeta({ 
  layout: 'landing',
})

// Initialize accordion logic for FlyonUI if needed interactively
onMounted(() => {
  // O FlyonUI cuida dos data-attributes através do script incluso no tailwind / nuxt pipeline.
  // Entretanto, como é Vue, garantimos hidratação tranquila.
  
  // Initialize FlyonUI accordion
  if (typeof window !== 'undefined') {
    // Wait for DOM to be ready
    const accordions = document.querySelectorAll('.accordion')
    accordions.forEach(accordion => {
      const items = accordion.querySelectorAll('.accordion-item')
      items.forEach(item => {
        const toggle = item.querySelector('.accordion-toggle')
        const content = item.querySelector('.accordion-content')
        if (toggle && content) {
          toggle.addEventListener('click', () => {
            const isActive = item.classList.contains('active')
            // Close all items in this accordion
            items.forEach(i => {
              i.classList.remove('active')
              const c = i.querySelector('.accordion-content')
              if (c) c.classList.add('hidden')
            })
            // Open clicked item if it was closed
            if (!isActive) {
              item.classList.add('active')
              content.classList.remove('hidden')
            }
          })
        }
      })
    })
  }
})
</script>

<style scoped>
/* Scoped overrides to protect the cleanliness of the layout */
.navbar {
  transition: background-color 0.3s ease, box-shadow 0.3s ease;
}
</style>