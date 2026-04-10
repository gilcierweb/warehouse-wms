<template>
  <div class="min-h-screen bg-base-100">
    <!-- Section 1: Navbar -->
    <nav class="navbar bg-base-100 shadow-sm fixed w-full z-50 top-0 border-b border-base-200">
      <div class="container mx-auto px-4">
        <div class="navbar-start">
          <a class="link link-neutral text-xl font-bold no-underline flex items-center gap-2" href="#">
            <span class="icon-[tabler--package] text-primary text-2xl"></span>
            WMS
          </a>
        </div>
        <div class="navbar-center hidden md:flex">
          <ul class="menu menu-horizontal gap-1">
            <li><a href="#features" class="rounded-lg hover:bg-base-200 px-3 py-2">Features</a></li>
            <li><a href="#how-it-works" class="rounded-lg hover:bg-base-200 px-3 py-2">How It Works</a></li>
            <li><a href="#pricing" class="rounded-lg hover:bg-base-200 px-3 py-2">Pricing</a></li>
            <li><a href="#faq" class="rounded-lg hover:bg-base-200 px-3 py-2">FAQ</a></li>
          </ul>
        </div>
        <div class="navbar-end gap-2">
          <div class="dropdown dropdown-end md:hidden">
            <button tabindex="0" role="button" class="btn btn-ghost btn-circle">
              <span class="icon-[tabler--menu-2] text-xl"></span>
            </button>
            <ul tabindex="0" class="dropdown-content menu z-[1] bg-base-100 shadow-lg rounded-box w-52 mt-4 p-2">
              <li><a href="#features">Features</a></li>
              <li><a href="#how-it-works">How It Works</a></li>
              <li><a href="#pricing">Pricing</a></li>
              <li><a href="#faq">FAQ</a></li>
            </ul>
          </div>
          <a href="#cta" class="btn btn-primary hidden md:inline-flex">
            Get Started
            <span class="icon-[tabler--arrow-right] ml-1"></span>
          </a>
        </div>
      </div>
    </nav>

    <!-- Section 2: Hero -->
    <section class="pt-24 pb-16 md:pt-32 md:pb-24 bg-gradient-to-br from-base-100 to-base-200">
      <div class="container mx-auto px-4">
        <div class="flex flex-col lg:flex-row items-center gap-12">
          <div class="flex-1 text-center lg:text-left">
            <div class="badge badge-primary badge-soft mb-4">
              <span class="icon-[tabler--sparkles] mr-1"></span>
              AI-Powered Analytics
            </div>
            <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-6 leading-tight">
              Warehouse Management
              <span class="text-primary block">Simplified</span>
            </h1>
            <p class="text-lg md:text-xl text-base-content/70 mb-8 max-w-xl mx-auto lg:mx-0">
              Streamline operations, reduce costs, and scale your business with intelligent inventory management.
            </p>
            <div class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
              <a href="#cta" class="btn btn-primary btn-lg">
                <span class="icon-[tabler--rocket] mr-2"></span>
                Start Free Trial
              </a>
              <button class="btn btn-outline btn-lg">
                <span class="icon-[tabler--player-play] mr-2"></span>
                Watch Demo
              </button>
            </div>
            <div class="mt-8 flex items-center gap-3 justify-center lg:justify-start">
              <div class="avatar-group -space-x-2">
                <div class="avatar">
                  <div class="w-8 h-8 rounded-full bg-primary text-primary-content text-xs font-bold flex items-center justify-center">JD</div>
                </div>
                <div class="avatar">
                  <div class="w-8 h-8 rounded-full bg-secondary text-secondary-content text-xs font-bold flex items-center justify-center">MK</div>
                </div>
                <div class="avatar">
                  <div class="w-8 h-8 rounded-full bg-accent text-accent-content text-xs font-bold flex items-center justify-center">SL</div>
                </div>
              </div>
              <span class="text-sm text-base-content/60">Trusted by <strong>500+</strong> warehouses</span>
            </div>
          </div>
          <div class="flex-1 w-full max-w-lg">
            <div class="mockup-browser bg-base-300 border border-base-300 shadow-2xl">
              <div class="mockup-browser-toolbar">
                <div class="input">https://wms.com/dashboard</div>
              </div>
              <div class="bg-base-100 p-4">
                <div class="grid grid-cols-3 gap-3 mb-4">
                  <div class="bg-primary/10 rounded-lg p-3 text-center">
                    <div class="text-xs text-base-content/60">Items</div>
                    <div class="text-lg font-bold text-primary">12.4K</div>
                  </div>
                  <div class="bg-success/10 rounded-lg p-3 text-center">
                    <div class="text-xs text-base-content/60">Accuracy</div>
                    <div class="text-lg font-bold text-success">98.5%</div>
                  </div>
                  <div class="bg-info/10 rounded-lg p-3 text-center">
                    <div class="text-xs text-base-content/60">Today</div>
                    <div class="text-lg font-bold text-info">342</div>
                  </div>
                </div>
                <div class="h-24 bg-gradient-to-r from-primary/20 to-secondary/20 rounded-lg flex items-end p-2 gap-1">
                  <div class="flex-1 bg-primary/60 rounded h-[40%]"></div>
                  <div class="flex-1 bg-primary/70 rounded h-[60%]"></div>
                  <div class="flex-1 bg-primary/80 rounded h-[80%]"></div>
                  <div class="flex-1 bg-primary rounded h-[55%]"></div>
                  <div class="flex-1 bg-primary/90 rounded h-[70%]"></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 3: Logo Cloud -->
    <section class="py-12 bg-base-200 border-y border-base-300">
      <div class="container mx-auto px-4">
        <p class="text-center text-sm font-medium text-base-content/50 mb-6 uppercase tracking-wider">Trusted by industry leaders</p>
        <div class="flex flex-wrap justify-center items-center gap-8 md:gap-16">
          <div class="flex items-center gap-2 text-xl font-bold text-base-content/40 hover:text-base-content/70 transition-colors">
            <span class="icon-[tabler--brand-amazon] text-3xl"></span>
            <span class="hidden sm:inline">Amazon</span>
          </div>
          <div class="flex items-center gap-2 text-xl font-bold text-base-content/40 hover:text-base-content/70 transition-colors">
            <span class="icon-[tabler--brand-fedex] text-3xl"></span>
            <span class="hidden sm:inline">FedEx</span>
          </div>
          <div class="flex items-center gap-2 text-xl font-bold text-base-content/40 hover:text-base-content/70 transition-colors">
            <span class="icon-[tabler--brand-ups] text-3xl"></span>
            <span class="hidden sm:inline">UPS</span>
          </div>
          <div class="flex items-center gap-2 text-xl font-bold text-base-content/40 hover:text-base-content/70 transition-colors">
            <span class="icon-[tabler--brand-dhl] text-3xl"></span>
            <span class="hidden sm:inline">DHL</span>
          </div>
          <div class="flex items-center gap-2 text-xl font-bold text-base-content/40 hover:text-base-content/70 transition-colors">
            <span class="icon-[tabler--brand-walmart] text-3xl"></span>
            <span class="hidden sm:inline">Walmart</span>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 4: Features -->
    <section id="features" class="py-16 md:py-24">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-3xl mx-auto mb-12 md:mb-16">
          <h2 class="text-3xl md:text-4xl font-bold mb-4">Everything you need to succeed</h2>
          <p class="text-lg text-base-content/70">Powerful features designed to streamline every aspect of warehouse management</p>
        </div>
        
        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-md transition-shadow">
            <div class="card-body">
              <div class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-4">
                <span class="icon-[tabler--package] text-2xl text-primary"></span>
              </div>
              <h3 class="card-title">Smart Inventory</h3>
              <p class="text-base-content/70">Real-time tracking with barcode scanning, automated stock alerts, and intelligent reordering.</p>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-md transition-shadow">
            <div class="card-body">
              <div class="w-12 h-12 bg-success/10 rounded-xl flex items-center justify-center mb-4">
                <span class="icon-[tabler--truck-delivery] text-2xl text-success"></span>
              </div>
              <h3 class="card-title">Movement Tracking</h3>
              <p class="text-base-content/70">Monitor inbound and outbound logistics with complete visibility of every shipment.</p>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-md transition-shadow">
            <div class="card-body">
              <div class="w-12 h-12 bg-info/10 rounded-xl flex items-center justify-center mb-4">
                <span class="icon-[tabler--chart-bar] text-2xl text-info"></span>
              </div>
              <h3 class="card-title">Analytics Dashboard</h3>
              <p class="text-base-content/70">Comprehensive reporting with customizable KPIs and predictive insights.</p>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-md transition-shadow">
            <div class="card-body">
              <div class="w-12 h-12 bg-warning/10 rounded-xl flex items-center justify-center mb-4">
                <span class="icon-[tabler--lock] text-2xl text-warning"></span>
              </div>
              <h3 class="card-title">Secure Access</h3>
              <p class="text-base-content/70">Role-based permissions, audit trails, and enterprise-grade security.</p>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-md transition-shadow">
            <div class="card-body">
              <div class="w-12 h-12 bg-secondary/10 rounded-xl flex items-center justify-center mb-4">
                <span class="icon-[tabler--api] text-2xl text-secondary"></span>
              </div>
              <h3 class="card-title">API Integration</h3>
              <p class="text-base-content/70">Connect with ERP, e-commerce, and shipping platforms seamlessly.</p>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-md transition-shadow">
            <div class="card-body">
              <div class="w-12 h-12 bg-accent/10 rounded-xl flex items-center justify-center mb-4">
                <span class="icon-[tabler--device-mobile] text-2xl text-accent"></span>
              </div>
              <h3 class="card-title">Mobile App</h3>
              <p class="text-base-content/70">Manage your warehouse on-the-go with our powerful mobile application.</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 5: How It Works -->
    <section id="how-it-works" class="py-16 md:py-24 bg-base-200">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-3xl mx-auto mb-12 md:mb-16">
          <h2 class="text-3xl md:text-4xl font-bold mb-4">Get started in minutes</h2>
          <p class="text-lg text-base-content/70">Simple setup process designed to get you up and running quickly</p>
        </div>
        
        <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <div class="card bg-base-100">
            <div class="card-body items-center text-center">
              <div class="w-14 h-14 rounded-full bg-primary text-primary-content flex items-center justify-center text-xl font-bold mb-4">1</div>
              <h3 class="card-title">Setup</h3>
              <p class="text-base-content/70 text-sm">Configure your warehouse layout and storage zones</p>
            </div>
          </div>
          
          <div class="card bg-base-100">
            <div class="card-body items-center text-center">
              <div class="w-14 h-14 rounded-full bg-primary text-primary-content flex items-center justify-center text-xl font-bold mb-4">2</div>
              <h3 class="card-title">Import</h3>
              <p class="text-base-content/70 text-sm">Add inventory with bulk upload or manual entry</p>
            </div>
          </div>
          
          <div class="card bg-base-100">
            <div class="card-body items-center text-center">
              <div class="w-14 h-14 rounded-full bg-primary text-primary-content flex items-center justify-center text-xl font-bold mb-4">3</div>
              <h3 class="card-title">Manage</h3>
              <p class="text-base-content/70 text-sm">Track movements and optimize operations</p>
            </div>
          </div>
          
          <div class="card bg-base-100">
            <div class="card-body items-center text-center">
              <div class="w-14 h-14 rounded-full bg-success text-success-content flex items-center justify-center text-xl font-bold mb-4">4</div>
              <h3 class="card-title">Optimize</h3>
              <p class="text-base-content/70 text-sm">Use analytics to improve efficiency</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 6: Stats -->
    <section class="py-16 bg-primary text-primary-content">
      <div class="container mx-auto px-4">
        <div class="stats shadow bg-transparent w-full flex flex-col lg:flex-row">
          <div class="stat place-items-center">
            <div class="stat-value text-4xl md:text-5xl">500+</div>
            <div class="stat-title text-primary-content/80">Warehouses</div>
          </div>
          <div class="stat place-items-center">
            <div class="stat-value text-4xl md:text-5xl">1M+</div>
            <div class="stat-title text-primary-content/80">Items Tracked</div>
          </div>
          <div class="stat place-items-center">
            <div class="stat-value text-4xl md:text-5xl">99.9%</div>
            <div class="stat-title text-primary-content/80">Uptime</div>
          </div>
          <div class="stat place-items-center">
            <div class="stat-value text-4xl md:text-5xl">50+</div>
            <div class="stat-title text-primary-content/80">Countries</div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 7: Testimonials -->
    <section class="py-16 md:py-24">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-3xl mx-auto mb-12 md:mb-16">
          <h2 class="text-3xl md:text-4xl font-bold mb-4">Loved by warehouse managers</h2>
          <p class="text-lg text-base-content/70">See what our customers have to say about their experience</p>
        </div>
        
        <div class="grid md:grid-cols-3 gap-6">
          <div class="card bg-base-100 border border-base-200">
            <div class="card-body">
              <div class="flex gap-1 mb-3">
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
              </div>
              <p class="text-base-content/80 mb-4">"This WMS transformed our warehouse operations. We've reduced inventory errors by 85% and saved countless hours."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-10 h-10 rounded-full bg-primary text-primary-content text-sm font-bold flex items-center justify-center">SJ</div>
                </div>
                <div>
                  <div class="font-bold text-sm">Sarah Johnson</div>
                  <div class="text-xs text-base-content/60">Operations Director</div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200">
            <div class="card-body">
              <div class="flex gap-1 mb-3">
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
              </div>
              <p class="text-base-content/80 mb-4">"The real-time tracking and analytics features are game-changers for our logistics. Highly recommend!"</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-10 h-10 rounded-full bg-secondary text-secondary-content text-sm font-bold flex items-center justify-center">MC</div>
                </div>
                <div>
                  <div class="font-bold text-sm">Michael Chen</div>
                  <div class="text-xs text-base-content/60">Warehouse Manager</div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200">
            <div class="card-body">
              <div class="flex gap-1 mb-3">
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
                <span class="icon-[tabler--star-filled] text-warning"></span>
              </div>
              <p class="text-base-content/80 mb-4">"Easy to implement and the support team is fantastic. Best investment we've made for our warehouse."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-10 h-10 rounded-full bg-accent text-accent-content text-sm font-bold flex items-center justify-center">ER</div>
                </div>
                <div>
                  <div class="font-bold text-sm">Emily Rodriguez</div>
                  <div class="text-xs text-base-content/60">Supply Chain Lead</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 8: Pricing -->
    <section id="pricing" class="py-16 md:py-24 bg-base-200">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-3xl mx-auto mb-12 md:mb-16">
          <h2 class="text-3xl md:text-4xl font-bold mb-4">Simple, transparent pricing</h2>
          <p class="text-lg text-base-content/70">Choose the plan that's right for your business</p>
        </div>
        
        <div class="grid md:grid-cols-3 gap-6 max-w-5xl mx-auto items-start">
          <div class="card bg-base-100 border border-base-200">
            <div class="card-body">
              <h3 class="card-title">Starter</h3>
              <div class="flex items-baseline gap-1 my-2">
                <span class="text-3xl font-bold">$49</span>
                <span class="text-base-content/60 text-sm">/month</span>
              </div>
              <p class="text-base-content/60 text-sm mb-4">Perfect for small warehouses</p>
              <ul class="space-y-2 mb-6">
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Up to 1,000 items
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  1 warehouse
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Basic reporting
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Email support
                </li>
              </ul>
              <button class="btn btn-outline w-full">Get Started</button>
            </div>
          </div>
          
          <div class="card bg-base-100 border-2 border-primary shadow-lg relative md:-mt-4">
            <div class="absolute -top-3 left-1/2 -translate-x-1/2">
              <span class="badge badge-primary badge-sm">Most Popular</span>
            </div>
            <div class="card-body">
              <h3 class="card-title">Professional</h3>
              <div class="flex items-baseline gap-1 my-2">
                <span class="text-3xl font-bold text-primary">$149</span>
                <span class="text-base-content/60 text-sm">/month</span>
              </div>
              <p class="text-base-content/60 text-sm mb-4">For growing operations</p>
              <ul class="space-y-2 mb-6">
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Up to 10,000 items
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  5 warehouses
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Advanced analytics
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Priority support
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  API access
                </li>
              </ul>
              <button class="btn btn-primary w-full">Get Started</button>
            </div>
          </div>
          
          <div class="card bg-base-100 border border-base-200">
            <div class="card-body">
              <h3 class="card-title">Enterprise</h3>
              <div class="flex items-baseline gap-1 my-2">
                <span class="text-3xl font-bold">Custom</span>
              </div>
              <p class="text-base-content/60 text-sm mb-4">For large-scale operations</p>
              <ul class="space-y-2 mb-6">
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Unlimited items
                </li>
                <li class="flex items-center gap-2 text-sm">
                  <span class="icon-[tabler--check] text-success"></span>
                  Unlimited warehouses
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
                  On-premise option
                </li>
              </ul>
              <button class="btn btn-outline w-full">Contact Sales</button>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 9: Integrations -->
    <section class="py-16">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-3xl mx-auto mb-12">
          <h2 class="text-3xl md:text-4xl font-bold mb-4">Integrations</h2>
          <p class="text-lg text-base-content/70">Connect with your favorite tools and platforms</p>
        </div>
        
        <div class="flex flex-wrap justify-center gap-4">
          <div class="badge badge-lg badge-soft badge-primary p-4 gap-2">
            <span class="icon-[tabler--building-store] text-xl"></span>
            ERP Systems
          </div>
          <div class="badge badge-lg badge-soft badge-secondary p-4 gap-2">
            <span class="icon-[tabler--shopping-cart] text-xl"></span>
            E-commerce
          </div>
          <div class="badge badge-lg badge-soft badge-success p-4 gap-2">
            <span class="icon-[tabler--truck] text-xl"></span>
            Shipping
          </div>
          <div class="badge badge-lg badge-soft badge-info p-4 gap-2">
            <span class="icon-[tabler--calculator] text-xl"></span>
            Accounting
          </div>
          <div class="badge badge-lg badge-soft badge-warning p-4 gap-2">
            <span class="icon-[tabler--users] text-xl"></span>
            CRM
          </div>
          <div class="badge badge-lg badge-soft badge-accent p-4 gap-2">
            <span class="icon-[tabler--chart-pie] text-xl"></span>
            BI Tools
          </div>
          <div class="badge badge-lg badge-soft badge-neutral p-4 gap-2">
            <span class="icon-[tabler--wifi] text-xl"></span>
            IoT Sensors
          </div>
          <div class="badge badge-lg badge-soft badge-primary p-4 gap-2">
            <span class="icon-[tabler--scan] text-xl"></span>
            Barcode Scanners
          </div>
        </div>
      </div>
    </section>

    <!-- Section 10: FAQ -->
    <section id="faq" class="py-16 md:py-24 bg-base-200">
      <div class="container mx-auto px-4 max-w-3xl">
        <div class="text-center mb-12">
          <h2 class="text-3xl md:text-4xl font-bold mb-4">Frequently asked questions</h2>
          <p class="text-lg text-base-content/70">Everything you need to know about our WMS</p>
        </div>
        
        <div class="join join-vertical w-full gap-2">
          <div class="collapse collapse-arrow join-item border border-base-200 bg-base-100 rounded-lg">
            <input type="radio" name="faq-accordion" /> 
            <div class="collapse-title font-medium">
              How long does implementation take?
            </div>
            <div class="collapse-content">
              <p class="text-base-content/70">Most implementations are completed within 2-4 weeks, depending on warehouse size and complexity. Our onboarding team will guide you through every step.</p>
            </div>
          </div>
          <div class="collapse collapse-arrow join-item border border-base-200 bg-base-100 rounded-lg">
            <input type="radio" name="faq-accordion" /> 
            <div class="collapse-title font-medium">
              Is training provided?
            </div>
            <div class="collapse-content">
              <p class="text-base-content/70">Yes! We provide comprehensive training including documentation, video tutorials, and live sessions. Professional and Enterprise plans include dedicated training sessions.</p>
            </div>
          </div>
          <div class="collapse collapse-arrow join-item border border-base-200 bg-base-100 rounded-lg">
            <input type="radio" name="faq-accordion" /> 
            <div class="collapse-title font-medium">
              Can I import existing inventory data?
            </div>
            <div class="collapse-content">
              <p class="text-base-content/70">Absolutely! We support bulk import from Excel, CSV, and direct integration with most major ERP systems and inventory management tools.</p>
            </div>
          </div>
          <div class="collapse collapse-arrow join-item border border-base-200 bg-base-100 rounded-lg">
            <input type="radio" name="faq-accordion" /> 
            <div class="collapse-title font-medium">
              Is there a mobile app available?
            </div>
            <div class="collapse-content">
              <p class="text-base-content/70">Yes, our mobile app is available for iOS and Android. It allows warehouse staff to scan, move, and track inventory on the go with full offline support.</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 11: CTA -->
    <section id="cta" class="py-16 md:py-24 bg-gradient-to-br from-primary to-secondary">
      <div class="container mx-auto px-4 text-center">
        <h2 class="text-3xl md:text-4xl lg:text-5xl font-bold text-primary-content mb-6">Ready to transform your warehouse?</h2>
        <p class="text-xl text-primary-content/80 mb-8 max-w-2xl mx-auto">Join thousands of warehouses already using our system. Start your free trial today.</p>
        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <button class="btn btn-lg bg-base-100 text-primary border-0 hover:bg-base-200 shadow-lg">
            <span class="icon-[tabler--rocket] mr-2"></span>
            Start Free Trial
          </button>
          <button class="btn btn-lg btn-ghost text-primary-content hover:bg-primary-content/10 border border-primary-content/20">
            <span class="icon-[tabler--calendar] mr-2"></span>
            Schedule Demo
          </button>
        </div>
        <p class="mt-8 text-sm text-primary-content/60">No credit card required. 14-day free trial.</p>
      </div>
    </section>

    <!-- Section 12: Footer -->
    <footer class="bg-neutral text-neutral-content py-12">
      <div class="container mx-auto px-4">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8 mb-8">
          <div class="col-span-2 md:col-span-1">
            <a class="flex items-center gap-2 text-xl font-bold mb-4" href="#">
              <span class="icon-[tabler--package] text-2xl"></span>
              WMS
            </a>
            <p class="text-neutral-content/70 text-sm mb-4">Modern warehouse management for modern businesses.</p>
            <div class="flex gap-2">
              <a href="#" class="btn btn-circle btn-sm btn-ghost">
                <span class="icon-[tabler--brand-twitter]"></span>
              </a>
              <a href="#" class="btn btn-circle btn-sm btn-ghost">
                <span class="icon-[tabler--brand-linkedin]"></span>
              </a>
              <a href="#" class="btn btn-circle btn-sm btn-ghost">
                <span class="icon-[tabler--brand-github]"></span>
              </a>
            </div>
          </div>
          
          <div>
            <h4 class="font-bold mb-4">Product</h4>
            <ul class="space-y-2 text-sm text-neutral-content/70">
              <li><a href="#features" class="hover:text-neutral-content">Features</a></li>
              <li><a href="#pricing" class="hover:text-neutral-content">Pricing</a></li>
              <li><a href="#" class="hover:text-neutral-content">Integrations</a></li>
              <li><a href="#" class="hover:text-neutral-content">API</a></li>
            </ul>
          </div>
          
          <div>
            <h4 class="font-bold mb-4">Company</h4>
            <ul class="space-y-2 text-sm text-neutral-content/70">
              <li><a href="#" class="hover:text-neutral-content">About</a></li>
              <li><a href="#" class="hover:text-neutral-content">Blog</a></li>
              <li><a href="#" class="hover:text-neutral-content">Careers</a></li>
              <li><a href="#" class="hover:text-neutral-content">Contact</a></li>
            </ul>
          </div>
          
          <div>
            <h4 class="font-bold mb-4">Support</h4>
            <ul class="space-y-2 text-sm text-neutral-content/70">
              <li><a href="#" class="hover:text-neutral-content">Help Center</a></li>
              <li><a href="#" class="hover:text-neutral-content">Documentation</a></li>
              <li><a href="#" class="hover:text-neutral-content">Status</a></li>
              <li><a href="#" class="hover:text-neutral-content">Security</a></li>
            </ul>
          </div>
        </div>
        
        <div class="border-t border-neutral-content/20 pt-8 flex flex-col sm:flex-row justify-between items-center gap-4">
          <p class="text-sm text-neutral-content/60">&copy; 2026 Warehouse WMS. All rights reserved.</p>
          <div class="flex gap-6 text-sm text-neutral-content/60">
            <a href="#" class="hover:text-neutral-content">Privacy Policy</a>
            <a href="#" class="hover:text-neutral-content">Terms of Service</a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
definePageMeta({ 
  layout: 'landing',
})
</script>

<style scoped>
/* Additional Landing Page Specific Styles */

/* Card Hover Effect */
.card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.card:hover {
  transform: translateY(-2px);
}

/* Mockup Browser Enhanced Shadow */
.mockup-browser {
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

/* Navbar Glassmorphism */
.navbar {
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
}

/* Stats Tabular Numbers */
.stat-value {
  font-variant-numeric: tabular-nums;
}

/* Badge Flex Alignment */
.badge {
  display: inline-flex;
  align-items: center;
}

/* Mobile Typography Scale */
@media (max-width: 640px) {
  h1 {
    font-size: 2.25rem !important;
  }
  h2 {
    font-size: 1.875rem !important;
  }
}
</style>