package cz.cvut.swa.bazaar.productservice.data

import java.math.BigDecimal

data class ProductDTO(

        val title: String,
        val description: String,
        val price: BigDecimal

)

data class ReviewDTO(

        val reviewer: String,
        val comment: String,
        val rating: Float

)

data class ProductStatusUpdateDTO(

        val id: String,
        val newStatus: ProductStatus

)