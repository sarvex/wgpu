[numthreads(1, 1, 1)]
void main()
{
    int2 i2_ = (int(0)).xx;
    int3 i3_ = (int(0)).xxx;
    int4 i4_ = (int(0)).xxxx;
    uint2 u2_ = (0u).xx;
    uint3 u3_ = (0u).xxx;
    uint4 u4_ = (0u).xxxx;
    float2 f2_ = (0.0).xx;
    float3 f3_ = (0.0).xxx;
    float4 f4_ = (0.0).xxxx;

    int2 _e27 = i2_;
    u2_ = asuint(_e27);
    int3 _e29 = i3_;
    u3_ = asuint(_e29);
    int4 _e31 = i4_;
    u4_ = asuint(_e31);
    uint2 _e33 = u2_;
    i2_ = asint(_e33);
    uint3 _e35 = u3_;
    i3_ = asint(_e35);
    uint4 _e37 = u4_;
    i4_ = asint(_e37);
    int2 _e39 = i2_;
    f2_ = asfloat(_e39);
    int3 _e41 = i3_;
    f3_ = asfloat(_e41);
    int4 _e43 = i4_;
    f4_ = asfloat(_e43);
    return;
}
